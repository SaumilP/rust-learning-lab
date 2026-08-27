//! # Data Processing Pipeline
//!
//! A demonstration of Rust's iterator chains and type-safe data processing,
//! comparing to Python's Pandas and list comprehensions.
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use data_pipeline::{UserPipeline, User};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Load data from CSV
//! let pipeline = UserPipeline::from_csv("users.csv")?;
//!
//! // Chain transformations (lazy evaluation)
//! let adults = pipeline
//!     .filter(|u| u.age >= 18)
//!     .filter(|u| u.is_gmail_user())
//!     .map(|mut u| {
//!         u.name = u.name.to_uppercase();
//!         u
//!     })
//!     .take(10);
//!
//! // Write results
//! adults.to_csv("output.csv")?;
//! # Ok(())
//! # }
//! ```

pub mod models;
pub mod pipeline;

// Re-export commonly used types
pub use models::{User, UserStats};
pub use pipeline::UserPipeline;
