//! # Async Database Client
//!
//! A demonstration of Rust's async/await ecosystem for database operations.
//! This project shows how to use SQLx and Tokio for asynchronous database access,
//! comparing the patterns to Java's Spring Data and CompletableFuture.
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use async_database_client::{DbConfig, Pool, UserRepository, NewUser};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create configuration
//!     let config = DbConfig::from_env()?;
//!
//!     // Create connection pool
//!     let pool = Pool::new(&config).await?;
//!     pool.run_migrations().await?;
//!
//!     // Create repository
//!     let repo = UserRepository::new(pool);
//!
//!     // Create a new user
//!     let new_user = NewUser::new(
//!         "alice@example.com".to_string(),
//!         "Alice Smith".to_string(),
//!         30,
//!     );
//!
//!     let user = repo.create(&new_user).await?;
//!     println!("Created user: {:?}", user);
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod error;
pub mod models;
pub mod pool;
pub mod repository;

// Re-export commonly used types
pub use config::DbConfig;
pub use error::{DbError, DbResult};
pub use models::{NewUser, UpdateUser, User};
pub use pool::{Pool, PoolStats};
pub use repository::UserRepository;
