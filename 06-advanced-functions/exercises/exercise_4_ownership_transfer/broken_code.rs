/// Broken Ownership Code - Fix the borrow checker errors!
/// There are 3 borrow checker violations

fn calculate_sum(nums: &Vec<i32>) -> i32 {
    nums.iter().sum()
}

fn double_values(nums: &mut Vec<i32>) {
    for num in nums.iter_mut() {
        *num *= 2;
    }
}

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];

    println!("Processing data...");
    println!("Created vector: {:?}", data);

    // ❌ BUG 1: Takes ownership, but data is used again later
    let sum = calculate_sum(data);
    println!("Sum: {}", sum);
    // println!("Average: {}", sum as f64 / data.len() as f64);  // ERROR: data was moved

    // ✓ Fix: Use reference &data instead of data

    // ❌ BUG 2: Multiple mutable borrows
    let ref1 = &mut data;
    let ref2 = &mut data;  // ERROR: Cannot borrow as mutable more than once
    ref1.push(10);
    ref2.push(20);

    // ✓ Fix: One mutable borrow at a time

    println!("Modified vector: {:?}", data);

    // ❌ BUG 3: Dangling reference
    let reference;
    {
        let temp_value = vec![1, 2, 3];
        reference = &temp_value;  // ERROR: temp_value doesn't live long enough
    }
    // println!("{:?}", reference);  // Would be dangling reference

    // ✓ Fix: Ensure referenced value outlives reference

    double_values(&mut data);
    println!("Processed: true");
}
