use crate::error::{DbError, DbResult};
use crate::models::{NewUser, UpdateUser, User};
use crate::pool::Pool;
use sqlx::Row;

/// Repository for user database operations
///
/// # Java Equivalent
/// ```java
/// @Repository
/// public class UserRepository {
///     @Autowired
///     private JdbcTemplate jdbcTemplate;
///     // ...
/// }
/// ```
#[derive(Clone)]
pub struct UserRepository {
    pool: Pool,
}

impl UserRepository {
    /// Create a new repository with the given pool
    pub fn new(pool: Pool) -> Self {
        UserRepository { pool }
    }

    /// Create a new user
    ///
    /// # Java Equivalent
    /// ```java
    /// public User create(NewUser newUser) {
    ///     String sql = "INSERT INTO users (email, name, age) VALUES (?, ?, ?)";
    ///     KeyHolder keyHolder = new GeneratedKeyHolder();
    ///     jdbcTemplate.update(connection -> {
    ///         PreparedStatement ps = connection.prepareStatement(sql, Statement.RETURN_GENERATED_KEYS);
    ///         ps.setString(1, newUser.getEmail());
    ///         ps.setString(2, newUser.getName());
    ///         ps.setInt(3, newUser.getAge());
    ///         return ps;
    ///     }, keyHolder);
    ///     return findById(keyHolder.getKey().longValue());
    /// }
    /// ```
    pub async fn create(&self, new_user: &NewUser) -> DbResult<User> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let result = sqlx::query(
            "INSERT INTO users (email, name, age, created_at) VALUES (?, ?, ?, ?)"
        )
        .bind(&new_user.email)
        .bind(&new_user.name)
        .bind(new_user.age)
        .bind(now)
        .execute(self.pool.inner())
        .await;

        match result {
            Ok(result) => {
                let id = result.last_insert_rowid();
                self.find_by_id(id).await
            }
            Err(e) => {
                if e.to_string().contains("UNIQUE constraint failed") {
                    Err(DbError::DuplicateUser(new_user.email.clone()))
                } else {
                    Err(DbError::DatabaseError(e))
                }
            }
        }
    }

    /// Find a user by ID
    ///
    /// # Java Equivalent
    /// ```java
    /// public Optional<User> findById(Long id) {
    ///     String sql = "SELECT * FROM users WHERE id = ?";
    ///     return jdbcTemplate.query(sql, userRowMapper, id)
    ///         .stream()
    ///         .findFirst();
    /// }
    /// ```
    pub async fn find_by_id(&self, id: i64) -> DbResult<User> {
        let row = sqlx::query("SELECT id, email, name, age, created_at FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(self.pool.inner())
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DbError::UserNotFound(format!("id: {}", id)),
                _ => DbError::DatabaseError(e),
            })?;

        Ok(User {
            id: row.get("id"),
            email: row.get("email"),
            name: row.get("name"),
            age: row.get("age"),
            created_at: row.get("created_at"),
        })
    }

    /// Find a user by email
    pub async fn find_by_email(&self, email: &str) -> DbResult<User> {
        let row = sqlx::query("SELECT id, email, name, age, created_at FROM users WHERE email = ?")
            .bind(email)
            .fetch_one(self.pool.inner())
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DbError::UserNotFound(email.to_string()),
                _ => DbError::DatabaseError(e),
            })?;

        Ok(User {
            id: row.get("id"),
            email: row.get("email"),
            name: row.get("name"),
            age: row.get("age"),
            created_at: row.get("created_at"),
        })
    }

    /// Find all users with age greater than the given value
    ///
    /// # Java Equivalent
    /// ```java
    /// public List<User> findByAgeGreaterThan(int age) {
    ///     String sql = "SELECT * FROM users WHERE age > ?";
    ///     return jdbcTemplate.query(sql, userRowMapper, age);
    /// }
    /// ```
    pub async fn find_by_age_greater_than(&self, age: i32) -> DbResult<Vec<User>> {
        let rows = sqlx::query("SELECT id, email, name, age, created_at FROM users WHERE age > ?")
            .bind(age)
            .fetch_all(self.pool.inner())
            .await?;

        let users = rows
            .iter()
            .map(|row| User {
                id: row.get("id"),
                email: row.get("email"),
                name: row.get("name"),
                age: row.get("age"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(users)
    }

    /// Find all users
    pub async fn find_all(&self) -> DbResult<Vec<User>> {
        let rows = sqlx::query("SELECT id, email, name, age, created_at FROM users ORDER BY id")
            .fetch_all(self.pool.inner())
            .await?;

        let users = rows
            .iter()
            .map(|row| User {
                id: row.get("id"),
                email: row.get("email"),
                name: row.get("name"),
                age: row.get("age"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(users)
    }

    /// Update a user
    pub async fn update(&self, id: i64, update: &UpdateUser) -> DbResult<User> {
        // First check if user exists
        let current = self.find_by_id(id).await?;

        let email = update.email.as_ref().unwrap_or(&current.email);
        let name = update.name.as_ref().unwrap_or(&current.name);
        let age = update.age.unwrap_or(current.age);

        sqlx::query("UPDATE users SET email = ?, name = ?, age = ? WHERE id = ?")
            .bind(email)
            .bind(name)
            .bind(age)
            .bind(id)
            .execute(self.pool.inner())
            .await?;

        self.find_by_id(id).await
    }

    /// Delete a user by ID
    pub async fn delete(&self, id: i64) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(self.pool.inner())
            .await?;

        if result.rows_affected() == 0 {
            Err(DbError::UserNotFound(format!("id: {}", id)))
        } else {
            Ok(())
        }
    }

    /// Count total users
    pub async fn count(&self) -> DbResult<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM users")
            .fetch_one(self.pool.inner())
            .await?;

        Ok(row.get("count"))
    }

    /// Delete all users (for testing)
    #[cfg(test)]
    pub async fn delete_all(&self) -> DbResult<()> {
        sqlx::query("DELETE FROM users")
            .execute(self.pool.inner())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DbConfig;

    async fn setup_test_db() -> UserRepository {
        let config = DbConfig::new("sqlite::memory:".to_string());
        let pool = Pool::new(&config).await.unwrap();
        pool.run_migrations().await.unwrap();
        UserRepository::new(pool)
    }

    #[tokio::test]
    async fn test_create_user() {
        let repo = setup_test_db().await;

        let new_user = NewUser::new(
            "test@example.com".to_string(),
            "Test User".to_string(),
            25,
        );

        let user = repo.create(&new_user).await.unwrap();
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.age, 25);
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let repo = setup_test_db().await;

        let new_user = NewUser::new(
            "find@example.com".to_string(),
            "Find User".to_string(),
            30,
        );
        let created = repo.create(&new_user).await.unwrap();

        let found = repo.find_by_id(created.id).await.unwrap();
        assert_eq!(found.id, created.id);
        assert_eq!(found.email, "find@example.com");
    }

    #[tokio::test]
    async fn test_find_by_email() {
        let repo = setup_test_db().await;

        let new_user = NewUser::new(
            "email@example.com".to_string(),
            "Email User".to_string(),
            28,
        );
        repo.create(&new_user).await.unwrap();

        let found = repo.find_by_email("email@example.com").await.unwrap();
        assert_eq!(found.email, "email@example.com");
    }

    #[tokio::test]
    async fn test_duplicate_email() {
        let repo = setup_test_db().await;

        let new_user = NewUser::new(
            "duplicate@example.com".to_string(),
            "User 1".to_string(),
            25,
        );
        repo.create(&new_user).await.unwrap();

        let duplicate = NewUser::new(
            "duplicate@example.com".to_string(),
            "User 2".to_string(),
            30,
        );
        let result = repo.create(&duplicate).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DbError::DuplicateUser(_)));
    }

    #[tokio::test]
    async fn test_update_user() {
        let repo = setup_test_db().await;

        let new_user = NewUser::new(
            "update@example.com".to_string(),
            "Original Name".to_string(),
            25,
        );
        let user = repo.create(&new_user).await.unwrap();

        let update = UpdateUser {
            email: None,
            name: Some("Updated Name".to_string()),
            age: Some(26),
        };

        let updated = repo.update(user.id, &update).await.unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.age, 26);
    }

    #[tokio::test]
    async fn test_delete_user() {
        let repo = setup_test_db().await;

        let new_user = NewUser::new(
            "delete@example.com".to_string(),
            "Delete User".to_string(),
            25,
        );
        let user = repo.create(&new_user).await.unwrap();

        repo.delete(user.id).await.unwrap();

        let result = repo.find_by_id(user.id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_all() {
        let repo = setup_test_db().await;

        repo.create(&NewUser::new("user1@example.com".to_string(), "User 1".to_string(), 25)).await.unwrap();
        repo.create(&NewUser::new("user2@example.com".to_string(), "User 2".to_string(), 30)).await.unwrap();

        let users = repo.find_all().await.unwrap();
        assert_eq!(users.len(), 2);
    }

    #[tokio::test]
    async fn test_find_by_age_greater_than() {
        let repo = setup_test_db().await;

        repo.create(&NewUser::new("young@example.com".to_string(), "Young".to_string(), 20)).await.unwrap();
        repo.create(&NewUser::new("old@example.com".to_string(), "Old".to_string(), 40)).await.unwrap();

        let users = repo.find_by_age_greater_than(25).await.unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].email, "old@example.com");
    }
}
