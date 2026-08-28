//! # Math Utilities Module
//!
//! This module provides comprehensive mathematical utilities for
//! common operations in Rust applications.
//!
//! ## Features
//!
//! - Basic arithmetic operations with overflow checking
//! - Statistical functions (mean, median, variance)
//! - Geometric calculations
//!
//! ## Quick Start
//!
//! ```rust
//! use math_utils::{arithmetic, statistics};
//!
//! // Basic arithmetic
//! let sum = arithmetic::add(5, 3);
//!
//! // Statistics
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let mean = statistics::mean(&data);
//! ```
//!
//! ## Module Organization
//!
//! - [`arithmetic`] - Basic math operations
//! - [`statistics`] - Statistical functions
//! - [`geometry`] - Geometric calculations
//!
//! ## Error Handling
//!
//! Functions that can fail return `Result<T, MathError>` or `Option<T>`.
//! See [`MathError`] for possible error conditions.
//!
//! ## Performance Notes
//!
//! All basic operations are O(1). Statistical functions are O(n)
//! where n is the size of the input data.

fn main() {
    println!("This file demonstrates module-level documentation.");
    println!("Generate docs with: cargo doc --open");
    println!();

    // Demo the module functions
    println!("=== Arithmetic Demo ===");
    println!("add(5, 3) = {:?}", arithmetic::add(5, 3));
    println!("safe_divide(10, 3) = {:?}", arithmetic::safe_divide(10, 3));

    println!("\n=== Statistics Demo ===");
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    println!("mean({:?}) = {:?}", data, statistics::mean(&data));

    println!("\n=== Geometry Demo ===");
    let circle = geometry::Circle::new(5.0);
    println!("Circle area (r=5) = {}", circle.area());
}

/// Basic arithmetic operations.
///
/// This module provides safe arithmetic operations that check for
/// overflow and handle edge cases properly.
///
/// # Examples
///
/// ```
/// use arithmetic::{add, safe_divide};
///
/// let sum = add(5, 3);
/// assert_eq!(sum, Ok(8));
///
/// let quotient = safe_divide(10, 2);
/// assert_eq!(quotient, Ok(5));
/// ```
pub mod arithmetic {
    use super::MathError;

    /// Adds two numbers with overflow checking.
    ///
    /// # Arguments
    ///
    /// * `a` - First operand
    /// * `b` - Second operand
    ///
    /// # Returns
    ///
    /// Returns `Ok(sum)` or `Err(MathError::Overflow)` if overflow occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = arithmetic::add(100, 50);
    /// assert_eq!(result, Ok(150));
    /// ```
    pub fn add(a: i32, b: i32) -> Result<i32, MathError> {
        a.checked_add(b).ok_or(MathError::Overflow)
    }

    /// Subtracts b from a with overflow checking.
    ///
    /// # Arguments
    ///
    /// * `a` - Number to subtract from
    /// * `b` - Number to subtract
    ///
    /// # Returns
    ///
    /// Returns `Ok(difference)` or `Err(MathError::Overflow)`.
    pub fn subtract(a: i32, b: i32) -> Result<i32, MathError> {
        a.checked_sub(b).ok_or(MathError::Overflow)
    }

    /// Multiplies two numbers with overflow checking.
    pub fn multiply(a: i32, b: i32) -> Result<i32, MathError> {
        a.checked_mul(b).ok_or(MathError::Overflow)
    }

    /// Safely divides a by b.
    ///
    /// # Errors
    ///
    /// Returns `Err(MathError::DivisionByZero)` if `b` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(arithmetic::safe_divide(10, 2), Ok(5));
    /// assert!(arithmetic::safe_divide(10, 0).is_err());
    /// ```
    pub fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
        if b == 0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
}

/// Statistical functions for data analysis.
///
/// Provides common statistical calculations for collections of numbers.
///
/// # Type Parameters
///
/// Most functions accept slices of `f64` values.
///
/// # Examples
///
/// ```
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
///
/// let avg = statistics::mean(&data);
/// let mid = statistics::median(&data);
/// let var = statistics::variance(&data);
/// ```
pub mod statistics {
    /// Calculates the arithmetic mean of a dataset.
    ///
    /// The mean is the sum of all values divided by the count.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of f64 values
    ///
    /// # Returns
    ///
    /// Returns `Some(mean)` or `None` if the slice is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// assert_eq!(statistics::mean(&data), Some(3.0));
    ///
    /// let empty: Vec<f64> = vec![];
    /// assert_eq!(statistics::mean(&empty), None);
    /// ```
    pub fn mean(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            return None;
        }
        let sum: f64 = data.iter().sum();
        Some(sum / data.len() as f64)
    }

    /// Calculates the median of a dataset.
    ///
    /// The median is the middle value when the data is sorted.
    /// For even-length datasets, it's the average of the two middle values.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of f64 values
    ///
    /// # Returns
    ///
    /// Returns `Some(median)` or `None` if the slice is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let odd = vec![1.0, 3.0, 2.0];  // Sorted: [1, 2, 3]
    /// assert_eq!(statistics::median(&odd), Some(2.0));
    ///
    /// let even = vec![1.0, 2.0, 3.0, 4.0];
    /// assert_eq!(statistics::median(&even), Some(2.5));
    /// ```
    pub fn median(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            return None;
        }

        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let len = sorted.len();
        if len % 2 == 0 {
            Some((sorted[len / 2 - 1] + sorted[len / 2]) / 2.0)
        } else {
            Some(sorted[len / 2])
        }
    }

    /// Calculates the variance of a dataset.
    ///
    /// Variance measures how spread out the data is from the mean.
    ///
    /// # Formula
    ///
    /// ```text
    /// variance = sum((x - mean)^2) / n
    /// ```
    ///
    /// # Returns
    ///
    /// Returns `Some(variance)` or `None` if the slice is empty.
    pub fn variance(data: &[f64]) -> Option<f64> {
        let mean = mean(data)?;
        let sum_sq_diff: f64 = data.iter().map(|x| (x - mean).powi(2)).sum();
        Some(sum_sq_diff / data.len() as f64)
    }

    /// Calculates the standard deviation.
    ///
    /// Standard deviation is the square root of variance.
    pub fn std_deviation(data: &[f64]) -> Option<f64> {
        variance(data).map(|v| v.sqrt())
    }
}

/// Geometric calculations and shapes.
///
/// This module provides types and functions for working with
/// common geometric shapes.
///
/// # Available Shapes
///
/// - [`geometry::Circle`] - Circular shape with radius
/// - [`geometry::Rectangle`] - Rectangular shape with width and height
///
/// # Examples
///
/// ```
/// let circle = geometry::Circle::new(5.0);
/// println!("Area: {}", circle.area());
/// println!("Circumference: {}", circle.circumference());
///
/// let rect = geometry::Rectangle::new(4.0, 3.0);
/// println!("Area: {}", rect.area());
/// println!("Perimeter: {}", rect.perimeter());
/// ```
pub mod geometry {
    /// A circle defined by its radius.
    ///
    /// # Examples
    ///
    /// ```
    /// let circle = geometry::Circle::new(10.0);
    ///
    /// assert_eq!(circle.radius(), 10.0);
    /// assert!((circle.area() - 314.159).abs() < 0.01);
    /// ```
    #[derive(Debug, Clone, Copy)]
    pub struct Circle {
        radius: f64,
    }

    impl Circle {
        /// Creates a new circle with the given radius.
        ///
        /// # Panics
        ///
        /// Panics if radius is negative.
        pub fn new(radius: f64) -> Self {
            assert!(radius >= 0.0, "Radius must be non-negative");
            Circle { radius }
        }

        /// Returns the radius.
        pub fn radius(&self) -> f64 {
            self.radius
        }

        /// Returns the diameter (2 * radius).
        pub fn diameter(&self) -> f64 {
            self.radius * 2.0
        }

        /// Calculates the area (pi * r^2).
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius.powi(2)
        }

        /// Calculates the circumference (2 * pi * r).
        pub fn circumference(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }

    /// A rectangle defined by width and height.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = geometry::Rectangle::new(4.0, 3.0);
    ///
    /// assert_eq!(rect.area(), 12.0);
    /// assert_eq!(rect.perimeter(), 14.0);
    /// ```
    #[derive(Debug, Clone, Copy)]
    pub struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        /// Creates a new rectangle.
        pub fn new(width: f64, height: f64) -> Self {
            Rectangle { width, height }
        }

        /// Returns the width.
        pub fn width(&self) -> f64 {
            self.width
        }

        /// Returns the height.
        pub fn height(&self) -> f64 {
            self.height
        }

        /// Calculates the area.
        pub fn area(&self) -> f64 {
            self.width * self.height
        }

        /// Calculates the perimeter.
        pub fn perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }

        /// Returns the diagonal length.
        pub fn diagonal(&self) -> f64 {
            (self.width.powi(2) + self.height.powi(2)).sqrt()
        }
    }
}

/// Errors that can occur during mathematical operations.
///
/// # Variants
///
/// | Error | Cause | Solution |
/// |-------|-------|----------|
/// | `DivisionByZero` | Divisor is zero | Check divisor before operation |
/// | `Overflow` | Result too large | Use larger type or check bounds |
/// | `InvalidInput` | Bad input data | Validate input |
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// Division by zero was attempted.
    DivisionByZero,
    /// Arithmetic overflow occurred.
    Overflow,
    /// Invalid input was provided.
    InvalidInput(String),
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::Overflow => write!(f, "Arithmetic overflow"),
            MathError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for MathError {}
