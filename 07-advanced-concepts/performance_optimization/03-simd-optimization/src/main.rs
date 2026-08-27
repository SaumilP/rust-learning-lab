// Stable Rust compatible performance optimization examples
// Note: This version uses optimized scalar operations instead of unstable SIMD
// The patterns shown here (loop unrolling, cache-friendly access) provide measurable performance benefits

mod simd_lib {
    pub use simd_optimization::*;
}

fn main() {
    println!("🚀 Performance Optimization Examples\n");

    // Example 1: Dot Product Comparison
    println!("=== 1. Dot Product Performance ===");
    let a: Vec<f32> = (0..1000).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..1000).map(|i| (i * 2) as f32).collect();

    let result_scalar = simd_lib::dot_product_scalar(&a, &b);
    let result_4way = simd_lib::dot_product_simd4(&a, &b);
    let result_8way = simd_lib::dot_product_simd8(&a, &b);

    println!("Scalar result: {}", result_scalar);
    println!("4-way optimized result: {}", result_4way);
    println!("8-way optimized result: {}", result_8way);
    assert_eq!(result_scalar, result_4way);
    assert_eq!(result_scalar, result_8way);

    // Example 2: Vector Addition
    println!("\n=== 2. Vector Addition ===");
    let result_scalar_add = simd_lib::vec_add_scalar(&a, &b);
    let result_simd_add = simd_lib::vec_add_simd(&a, &b);

    println!("Scalar addition: first 5 elements = {:?}", &result_scalar_add[0..5]);
    println!("Optimized addition: first 5 elements = {:?}", &result_simd_add[0..5]);
    assert_eq!(result_scalar_add, result_simd_add);

    // Example 3: Sum of Squares
    println!("\n=== 3. Sum of Squares ===");
    let data: Vec<f32> = (1..=100).map(|i| i as f32).collect();

    let sum_scalar = simd_lib::sum_of_squares_scalar(&data);
    let sum_simd = simd_lib::sum_of_squares_simd(&data);

    println!("Scalar sum of squares: {}", sum_scalar);
    println!("Optimized sum of squares: {}", sum_simd);
    assert_eq!(sum_scalar, sum_simd);

    // Example 4: Optimization Techniques Explained
    println!("\n=== 4. Optimization Techniques ===");
    println!("✓ Loop Unrolling: Process multiple iterations per loop");
    println!("  - Reduces loop overhead");
    println!("  - Improves CPU instruction pipeline efficiency");
    println!("  - Example: 4-way and 8-way unrolling above");

    println!("\n✓ Dependency Breaking: Using separate accumulators");
    println!("  - Allows CPU to execute iterations in parallel");
    println!("  - Increases instruction-level parallelism (ILP)");

    println!("\n✓ Memory Access Patterns: Sequential access");
    println!("  - Better cache locality");
    println!("  - Fewer cache misses");

    println!("\n✓ SIMD (Single Instruction Multiple Data):");
    println!("  - On stable Rust, use loop unrolling for similar benefits");
    println!("  - Consider crates like 'packed_simd' or 'wide' for actual SIMD");
    println!("  - Or use nightly Rust with std::simd for cutting-edge SIMD");

    // Example 5: Large-scale performance test
    println!("\n=== 5. Large-scale Performance Test ===");
    let large_a: Vec<f32> = (0..100000).map(|i| (i as f32).sin()).collect();
    let large_b: Vec<f32> = (0..100000).map(|i| (i as f32).cos()).collect();

    let _ = simd_lib::dot_product_scalar(&large_a, &large_b);
    let _ = simd_lib::dot_product_simd4(&large_a, &large_b);
    let _ = simd_lib::dot_product_simd8(&large_a, &large_b);
    println!("✓ Successfully computed dot products with 100k elements");
    println!("  (Performance benchmarking should be done with criterion crate)");

    println!("\n✅ All optimization examples completed!");
}
