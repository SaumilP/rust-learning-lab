use crate::models::{User, UserStats};
use csv::{Reader, Writer};
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

/// Data processing pipeline for users
///
/// # Python Equivalent
/// ```python
/// import pandas as pd
///
/// class UserPipeline:
///     def __init__(self, file_path):
///         self.df = pd.read_csv(file_path)
/// ```
#[derive(Clone)]
pub struct UserPipeline {
    users: Vec<User>,
}

impl UserPipeline {
    /// Load users from CSV file
    ///
    /// # Python Equivalent
    /// ```python
    /// pipeline = UserPipeline('users.csv')
    /// ```
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut reader = Reader::from_path(path)?;
        let users: Vec<User> = reader
            .deserialize()
            .collect::<Result<_, _>>()?;

        Ok(UserPipeline { users })
    }

    /// Create pipeline from vector of users
    pub fn from_users(users: Vec<User>) -> Self {
        UserPipeline { users }
    }

    /// Filter users by predicate
    ///
    /// # Python Equivalent
    /// ```python
    /// df[df['age'] >= 18]
    /// ```
    pub fn filter<F>(self, predicate: F) -> Self
    where
        F: Fn(&User) -> bool,
    {
        let users = self.users.into_iter().filter(predicate).collect();
        UserPipeline { users }
    }

    /// Transform users using a function
    ///
    /// # Python Equivalent
    /// ```python
    /// df['name'] = df['name'].str.upper()
    /// ```
    pub fn map<F>(self, transform: F) -> Self
    where
        F: Fn(User) -> User,
    {
        let users = self.users.into_iter().map(transform).collect();
        UserPipeline { users }
    }

    /// Sort users by a key function
    ///
    /// # Python Equivalent
    /// ```python
    /// df.sort_values('age', ascending=False)
    /// ```
    pub fn sort_by<F, K>(mut self, key_fn: F) -> Self
    where
        F: Fn(&User) -> K,
        K: Ord,
    {
        self.users.sort_by_key(key_fn);
        self
    }

    /// Take first n users
    ///
    /// # Python Equivalent
    /// ```python
    /// df.head(n)
    /// ```
    pub fn take(self, n: usize) -> Self {
        let users = self.users.into_iter().take(n).collect();
        UserPipeline { users }
    }

    /// Group users by a key and apply aggregation
    ///
    /// # Python Equivalent
    /// ```python
    /// df.groupby('city')['age'].mean()
    /// ```
    pub fn group_by<F, K>(&self, key_fn: F) -> HashMap<K, UserStats>
    where
        F: Fn(&User) -> K,
        K: std::hash::Hash + Eq,
    {
        use itertools::Itertools;

        self.users
            .iter()
            .into_group_map_by(|u| key_fn(u))
            .into_iter()
            .filter_map(|(key, users)| {
                let users_vec: Vec<_> = users.into_iter().cloned().collect();
                UserStats::from_users(&users_vec).map(|stats| (key, stats))
            })
            .collect()
    }

    /// Process users in parallel
    ///
    /// # Python Equivalent
    /// ```python
    /// from multiprocessing import Pool
    /// with Pool(4) as pool:
    ///     results = pool.map(transform, users)
    /// ```
    pub fn parallel_map<F>(self, transform: F) -> Self
    where
        F: Fn(User) -> User + Sync + Send,
        User: Send,
    {
        let users = self.users.into_par_iter().map(transform).collect();
        UserPipeline { users }
    }

    /// Get the users
    pub fn users(&self) -> &[User] {
        &self.users
    }

    /// Write users to CSV file
    ///
    /// # Python Equivalent
    /// ```python
    /// df.to_csv('output.csv', index=False)
    /// ```
    pub fn to_csv<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let mut writer = Writer::from_path(path)?;

        for user in &self.users {
            writer.serialize(user)?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Write users to JSON file
    pub fn to_json<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(&self.users)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Get count of users
    pub fn count(&self) -> usize {
        self.users.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_users() -> Vec<User> {
        vec![
            User {
                name: "Alice".to_string(),
                age: 30,
                email: "alice@gmail.com".to_string(),
                city: "NYC".to_string(),
            },
            User {
                name: "Bob".to_string(),
                age: 25,
                email: "bob@yahoo.com".to_string(),
                city: "LA".to_string(),
            },
            User {
                name: "Carol".to_string(),
                age: 35,
                email: "carol@gmail.com".to_string(),
                city: "NYC".to_string(),
            },
        ]
    }

    #[test]
    fn test_filter() {
        let pipeline = UserPipeline::from_users(create_test_users());
        let filtered = pipeline.filter(|u| u.age >= 30);
        assert_eq!(filtered.count(), 2);
    }

    #[test]
    fn test_map() {
        let pipeline = UserPipeline::from_users(create_test_users());
        let transformed = pipeline.map(|mut u| {
            u.name = u.name.to_uppercase();
            u
        });
        assert_eq!(transformed.users()[0].name, "ALICE");
    }

    #[test]
    fn test_chain() {
        let pipeline = UserPipeline::from_users(create_test_users());
        let result = pipeline
            .filter(|u| u.is_gmail_user())
            .map(|mut u| {
                u.name = u.name.to_uppercase();
                u
            })
            .take(1);

        assert_eq!(result.count(), 1);
        assert_eq!(result.users()[0].name, "ALICE");
    }

    #[test]
    fn test_group_by() {
        let pipeline = UserPipeline::from_users(create_test_users());
        let groups = pipeline.group_by(|u| u.city.clone());

        assert_eq!(groups.len(), 2);
        assert!(groups.contains_key("NYC"));
        assert_eq!(groups["NYC"].count, 2);
    }
}
