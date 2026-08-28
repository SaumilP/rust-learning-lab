// Exercise 2: Add Documentation and Pass Doc Tests (BROKEN CODE)
//
// This code has missing or broken documentation.
// Your job: Add proper doc comments with working examples.
// The doc tests (examples in comments) must compile and pass.

// BUG 1: Module documentation is missing
// Add module-level docs using //! comments

// Missing: Module description explaining what this library provides

/// BUG 2: This doc comment has a broken example
/// The example code won't compile because of a typo
///
/// # Examples
///
/// ```
/// let result = gcd(48, 18);  // Missing module path or use statement
/// assert_eq!(result, 6);
/// ```
pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// BUG 3: Missing documentation entirely
// This function has no doc comments at all
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// BUG 4: Documentation has wrong assertion in example
/// The example will fail because the expected value is incorrect
///
/// # Examples
///
/// ```
/// let result = fizzbuzz(15);
/// assert_eq!(result, "Fizz");  // Wrong! Should be "FizzBuzz"
/// ```
pub fn fizzbuzz(n: u32) -> String {
    match (n % 3 == 0, n % 5 == 0) {
        (true, true) => "FizzBuzz".to_string(),
        (true, false) => "Fizz".to_string(),
        (false, true) => "Buzz".to_string(),
        (false, false) => n.to_string(),
    }
}

// BUG 5: Missing documentation, needs examples showing conversion
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

// BUGS SUMMARY:
// 1. No module-level documentation (//! comments at top of file)
// 2. gcd example doesn't work - need to reference the function correctly
//    In doc tests, need to use the crate name or full path
// 3. is_prime has no documentation at all
// 4. fizzbuzz example has wrong expected value (Fizz instead of FizzBuzz)
// 5. celsius_to_fahrenheit has no documentation

// FIXES NEEDED:
// 1. Add //! module docs at file start
// 2. Fix gcd example to properly call the function:
//    ```
//    use your_crate_name::gcd;  // or just show the function inline
//    # fn gcd(a: u32, b: u32) -> u32 { if b == 0 { a } else { gcd(b, a % b) } }
//    assert_eq!(gcd(48, 18), 6);
//    ```
// 3. Add full documentation to is_prime with examples
// 4. Fix fizzbuzz assertion: fizzbuzz(15) should equal "FizzBuzz"
// 5. Add documentation to celsius_to_fahrenheit

// To test doc tests:
// cargo test --doc
//
// To generate documentation:
// cargo doc --open
