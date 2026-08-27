use my_attribute::{log_entry_exit, time_execution, retry, deprecated_fn};
use std::thread;
use std::time::Duration;

// Example 1: Log entry and exit
#[log_entry_exit]
fn greet(name: &str) -> String {
    println!("  Processing greeting...");
    format!("Hello, {}!", name)
}

// Example 2: Time execution
#[time_execution]
fn expensive_calculation() -> u64 {
    thread::sleep(Duration::from_millis(100));
    let result: u64 = (1..=1000).sum();
    result
}

// Example 3: Combine multiple attributes
#[log_entry_exit]
#[time_execution]
fn combined_example() {
    thread::sleep(Duration::from_millis(50));
    println!("  Doing some work...");
}

// Example 4: Retry logic
#[retry(times = 3, delay_ms = 50)]
fn flaky_network_call(should_fail: bool) -> Result<String, String> {
    if should_fail {
        Err("Network error".to_string())
    } else {
        Ok("Success!".to_string())
    }
}

// Example 5: Deprecated function
#[deprecated_fn(since = "1.2.0", note = "Use new_api instead")]
fn old_api() -> &'static str {
    "Old API response"
}

fn new_api() -> &'static str {
    "New API response"
}

fn main() {
    println!("🎨 Attribute Macro Examples\n");

    // Example 1
    println!("=== 1. Log Entry/Exit ===");
    let greeting = greet("Alice");
    println!("Result: {}\n", greeting);

    // Example 2
    println!("=== 2. Time Execution ===");
    let result = expensive_calculation();
    println!("Result: {}\n", result);

    // Example 3
    println!("=== 3. Combined Attributes ===");
    combined_example();
    println!();

    // Example 4
    println!("=== 4. Retry Logic ===");
    println!("Successful call:");
    match flaky_network_call(false) {
        Ok(msg) => println!("  {}", msg),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\nFailing call (will retry):");
    match flaky_network_call(true) {
        Ok(msg) => println!("  {}", msg),
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Example 5
    println!("=== 5. Deprecated Function ===");
    #[allow(deprecated)]
    {
        let old_result = old_api();
        println!("Old API: {}", old_result);
    }
    let new_result = new_api();
    println!("New API: {}", new_result);

    println!("\n✅ All attribute macro examples completed!");
}
