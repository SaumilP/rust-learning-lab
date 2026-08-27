use crate::error::{DbError, DbResult};
use std::env;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub idle_timeout_secs: u64,
}

impl DbConfig {
    /// Create a new configuration from environment variables
    pub fn from_env() -> DbResult<Self> {
        dotenv::dotenv().ok(); // Load .env file if it exists

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| DbError::ConfigError("DATABASE_URL not set".to_string()))?;

        let max_connections = env::var("MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);

        let min_connections = env::var("MIN_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);

        let connect_timeout_secs = env::var("CONNECT_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);

        let idle_timeout_secs = env::var("IDLE_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(600);

        Ok(DbConfig {
            database_url,
            max_connections,
            min_connections,
            connect_timeout_secs,
            idle_timeout_secs,
        })
    }

    /// Create a new configuration with custom values
    pub fn new(database_url: String) -> Self {
        DbConfig {
            database_url,
            max_connections: 5,
            min_connections: 1,
            connect_timeout_secs: 5,
            idle_timeout_secs: 600,
        }
    }

    /// Set maximum number of connections
    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// Set minimum number of connections
    pub fn with_min_connections(mut self, min: u32) -> Self {
        self.min_connections = min;
        self
    }

    /// Set connection timeout in seconds
    pub fn with_connect_timeout(mut self, secs: u64) -> Self {
        self.connect_timeout_secs = secs;
        self
    }

    /// Set idle timeout in seconds
    pub fn with_idle_timeout(mut self, secs: u64) -> Self {
        self.idle_timeout_secs = secs;
        self
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig {
            database_url: "sqlite::memory:".to_string(),
            max_connections: 5,
            min_connections: 1,
            connect_timeout_secs: 5,
            idle_timeout_secs: 600,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DbConfig::default();
        assert_eq!(config.database_url, "sqlite::memory:");
        assert_eq!(config.max_connections, 5);
    }

    #[test]
    fn test_config_builder() {
        let config = DbConfig::new("sqlite:test.db".to_string())
            .with_max_connections(10)
            .with_min_connections(2)
            .with_connect_timeout(10);

        assert_eq!(config.database_url, "sqlite:test.db");
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 2);
        assert_eq!(config.connect_timeout_secs, 10);
    }
}
