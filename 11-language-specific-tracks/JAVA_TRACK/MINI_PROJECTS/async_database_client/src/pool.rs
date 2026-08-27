use crate::config::DbConfig;
use crate::error::{DbError, DbResult};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use std::time::Duration;

/// Database connection pool wrapper
///
/// This wraps SQLx's connection pool and provides additional functionality
/// like health checks and connection management.
#[derive(Clone)]
pub struct Pool {
    inner: SqlitePool,
}

impl Pool {
    /// Create a new connection pool from configuration
    ///
    /// # Java Equivalent
    /// ```java
    /// HikariConfig config = new HikariConfig();
    /// config.setJdbcUrl(url);
    /// config.setMaximumPoolSize(maxConnections);
    /// HikariDataSource pool = new HikariDataSource(config);
    /// ```
    pub async fn new(config: &DbConfig) -> DbResult<Self> {
        let connect_options = SqliteConnectOptions::from_str(&config.database_url)
            .map_err(|e| DbError::ConfigError(format!("Invalid database URL: {}", e)))?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.connect_timeout_secs))
            .idle_timeout(Duration::from_secs(config.idle_timeout_secs))
            .connect_with(connect_options)
            .await?;

        Ok(Pool { inner: pool })
    }

    /// Get a reference to the inner pool
    pub fn inner(&self) -> &SqlitePool {
        &self.inner
    }

    /// Run database migrations
    ///
    /// This creates the necessary tables if they don't exist.
    pub async fn run_migrations(&self) -> DbResult<()> {
        // Create users table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                age INTEGER NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            )
            "#,
        )
        .execute(&self.inner)
        .await?;

        // Create indexes
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)")
            .execute(&self.inner)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_age ON users(age)")
            .execute(&self.inner)
            .await?;

        Ok(())
    }

    /// Check if the pool is healthy (can acquire a connection)
    pub async fn health_check(&self) -> DbResult<bool> {
        match sqlx::query("SELECT 1").fetch_one(&self.inner).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        PoolStats {
            size: self.inner.size(),
            idle_connections: self.inner.num_idle(),
        }
    }

    /// Close the pool
    pub async fn close(&self) {
        self.inner.close().await;
    }
}

/// Statistics about the connection pool
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub size: u32,
    pub idle_connections: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_creation() {
        let config = DbConfig::new("sqlite::memory:".to_string());
        let pool = Pool::new(&config).await.unwrap();

        assert!(pool.health_check().await.unwrap());
    }

    #[tokio::test]
    async fn test_pool_stats() {
        let config = DbConfig::new("sqlite::memory:".to_string());
        let pool = Pool::new(&config).await.unwrap();

        let stats = pool.stats();
        assert!(stats.size > 0);
    }

    #[tokio::test]
    async fn test_migrations() {
        let config = DbConfig::new("sqlite::memory:".to_string());
        let pool = Pool::new(&config).await.unwrap();

        // Should not fail
        pool.run_migrations().await.unwrap();

        // Running again should be idempotent
        pool.run_migrations().await.unwrap();
    }
}
