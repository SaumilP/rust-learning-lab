# SIMD Optimization Examples

Demonstrate vectorization using Rust's portable SIMD for performance gains.

## What is SIMD?

SIMD (Single Instruction, Multiple Data) allows processing multiple data elements in parallel using special CPU instructions. This can provide 2-8x speedup for data-parallel operations.

## Examples Included

1. **Dot Product** - Scalar vs SIMD (4-wide and 8-wide)
2. **Vector Addition** - Element-wise addition
3. **Sum of Squares** - Common mathematical operation

## Running

```bash
# Run examples
cargo run --release

# Run benchmarks
cargo bench

# View benchmark report
open target/criterion/report/index.html
```

## Expected Speedups

On modern CPUs (AVX2/NEON):
- SIMD4 (128-bit): ~2-4x faster
- SIMD8 (256-bit): ~4-8x faster

## Key Concepts

### SIMD Types

```rust
use std::simd::{f32x4, f32x8};

// 4 floats processed in parallel
let a = f32x4::splat(1.0);
let b = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
let c = a + b;  // Parallel addition
```

### Loading Data

```rust
// From slice
let vec = f32x8::from_slice(&data[i * 8..]);

// From array
let vec = f32x8::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);

// Splat (broadcast)
let vec = f32x8::splat(42.0);  // All lanes = 42.0
```

### Arithmetic Operations

```rust
let a = f32x8::from_slice(&data_a);
let b = f32x8::from_slice(&data_b);

// Element-wise operations
let sum = a + b;
let product = a * b;
let diff = a - b;
```

### Reductions

```rust
let vec = f32x8::from_slice(&data);

// Sum all lanes
let total = vec.reduce_sum();

// Min/max
let min = vec.reduce_min();
let max = vec.reduce_max();
```

## Best Practices

1. **Process in chunks** - Handle remainder separately
2. **Align data** - 16/32-byte alignment helps
3. **Benchmark** - Measure actual performance gain
4. **Profile** - Verify SIMD instructions are used
5. **Fallback** - Provide scalar implementation

## When to Use SIMD

Good for:
- Mathematical operations on arrays
- Image/video processing
- Audio processing
- Physics simulations
- Data transformations

Not worth it for:
- Small datasets (< 100 elements)
- Complex branching logic
- Non-contiguous data
- Already memory-bound code

## Platform Support

Rust's portable SIMD works on:
- x86/x64 (SSE, AVX, AVX-512)
- ARM (NEON)
- WASM (SIMD128)

Compiler automatically selects best instructions for target CPU.

## Verifying SIMD Usage

```bash
# Check assembly output
cargo asm --release simd_optimization::dot_product_simd8

# Or use godbolt.org
# Look for: vmulps, vaddps (AVX), etc.
```
