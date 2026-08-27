use serde::{Deserialize, Serialize};

/// User model representing a row in the users table
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub age: i32,
    pub created_at: i64,
}

/// Data for creating a new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub name: String,
    pub age: i32,
}

/// Data for updating an existing user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub name: Option<String>,
    pub age: Option<i32>,
}

impl User {
    /// Create a new User instance
    pub fn new(id: i64, email: String, name: String, age: i32, created_at: i64) -> Self {
        User {
            id,
            email,
            name,
            age,
            created_at,
        }
    }
}

impl NewUser {
    /// Create a new NewUser instance
    pub fn new(email: String, name: String, age: i32) -> Self {
        NewUser { email, name, age }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user_creation() {
        let new_user = NewUser::new(
            "test@example.com".to_string(),
            "Test User".to_string(),
            25,
        );
        assert_eq!(new_user.email, "test@example.com");
        assert_eq!(new_user.name, "Test User");
        assert_eq!(new_user.age, 25);
    }

    #[test]
    fn test_user_creation() {
        let user = User::new(
            1,
            "test@example.com".to_string(),
            "Test User".to_string(),
            25,
            1234567890,
        );
        assert_eq!(user.id, 1);
        assert_eq!(user.email, "test@example.com");
    }
}
