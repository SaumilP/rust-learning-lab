use thiserror::Error;

/// Custom error types for database operations
#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Duplicate user: {0}")]
    DuplicateUser(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Migration error: {0}")]
    MigrationError(String),
}

/// Result type alias for database operations
pub type DbResult<T> = Result<T, DbError>;

impl DbError {
    /// Check if error is a constraint violation (unique constraint)
    pub fn is_unique_violation(&self) -> bool {
        match self {
            DbError::DatabaseError(sqlx::Error::Database(db_err)) => {
                db_err.message().contains("UNIQUE constraint failed")
                    || db_err.message().contains("duplicate key")
            }
            _ => false,
        }
    }

    /// Check if error is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(
            self,
            DbError::UserNotFound(_) | DbError::DatabaseError(sqlx::Error::RowNotFound)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_not_found_error() {
        let error = DbError::UserNotFound("test@example.com".to_string());
        assert!(error.is_not_found());
        assert_eq!(error.to_string(), "User not found: test@example.com");
    }

    #[test]
    fn test_invalid_input_error() {
        let error = DbError::InvalidInput("Age must be positive".to_string());
        assert_eq!(error.to_string(), "Invalid input: Age must be positive");
    }
}
