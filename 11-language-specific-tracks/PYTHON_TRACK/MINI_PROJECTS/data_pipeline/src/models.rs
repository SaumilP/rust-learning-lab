use serde::{Deserialize, Serialize};
use std::fmt;

/// User record from CSV file
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub email: String,
    pub city: String,
}

impl User {
    /// Create a new user with validation
    pub fn new(name: String, age: u32, email: String, city: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if !email.contains('@') {
            return Err(format!("Invalid email: {}", email));
        }
        if age > 150 {
            return Err(format!("Invalid age: {}", age));
        }
        Ok(User {
            name,
            age,
            email,
            city,
        })
    }

    /// Check if user has a Gmail address
    pub fn is_gmail_user(&self) -> bool {
        self.email.ends_with("@gmail.com")
    }

    /// Check if user is an adult
    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, {}) - {}",
            self.name, self.age, self.city, self.email
        )
    }
}

/// Aggregated statistics for a group of users
#[derive(Debug, Clone, Serialize)]
pub struct UserStats {
    pub count: usize,
    pub avg_age: f64,
    pub min_age: u32,
    pub max_age: u32,
}

impl UserStats {
    /// Calculate statistics from a slice of users
    pub fn from_users(users: &[User]) -> Option<Self> {
        if users.is_empty() {
            return None;
        }

        let count = users.len();
        let sum_age: u32 = users.iter().map(|u| u.age).sum();
        let avg_age = sum_age as f64 / count as f64;
        let min_age = users.iter().map(|u| u.age).min().unwrap();
        let max_age = users.iter().map(|u| u.age).max().unwrap();

        Some(UserStats {
            count,
            avg_age,
            min_age,
            max_age,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "Alice".to_string(),
            30,
            "alice@example.com".to_string(),
            "NYC".to_string(),
        );
        assert!(user.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let user = User::new(
            "Bob".to_string(),
            25,
            "invalid-email".to_string(),
            "LA".to_string(),
        );
        assert!(user.is_err());
    }

    #[test]
    fn test_is_gmail_user() {
        let user = User {
            name: "Alice".to_string(),
            age: 30,
            email: "alice@gmail.com".to_string(),
            city: "NYC".to_string(),
        };
        assert!(user.is_gmail_user());
    }

    #[test]
    fn test_user_stats() {
        let users = vec![
            User {
                name: "Alice".to_string(),
                age: 30,
                email: "alice@example.com".to_string(),
                city: "NYC".to_string(),
            },
            User {
                name: "Bob".to_string(),
                age: 25,
                email: "bob@example.com".to_string(),
                city: "LA".to_string(),
            },
        ];

        let stats = UserStats::from_users(&users).unwrap();
        assert_eq!(stats.count, 2);
        assert_eq!(stats.avg_age, 27.5);
        assert_eq!(stats.min_age, 25);
        assert_eq!(stats.max_age, 30);
    }
}
