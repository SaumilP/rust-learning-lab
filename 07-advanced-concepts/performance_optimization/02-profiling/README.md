# Profiling Tools and Techniques

Example program with intentionally inefficient code for profiling practice.

## Profiling Tools

### 1. Flamegraph (CPU Profiling)

```bash
# Install
cargo install flamegraph

# Generate flamegraph
cargo flamegraph

# Open flamegraph.svg in browser
```

Flamegraphs show:
- Which functions consume the most CPU time
- Call stack visualization
- Hot paths in your code

### 2. perf (Linux CPU Profiling)

```bash
# Build in release mode
cargo build --release

# Record
perf record --call-graph=dwarf ./target/release/profiling

# View report
perf report

# Show statistics
perf stat ./target/release/profiling
```

### 3. Valgrind Massif (Memory Profiling)

```bash
# Run with massif
valgrind --tool=massif ./target/release/profiling

# Visualize
ms_print massif.out.*

# Or use massif-visualizer (GUI)
massif-visualizer massif.out.*
```

Shows:
- Heap usage over time
- Allocation hot spots
- Memory leaks

### 4. Valgrind Cachegrind (Cache Profiling)

```bash
# Run with cachegrind
valgrind --tool=cachegrind ./target/release/profiling

# View results
cg_annotate cachegrind.out.*
```

Shows:
- Cache hit/miss rates
- Branch prediction performance
- CPU instruction counts

### 5. DHAT (Heap Profiling)

```bash
# Run with DHAT
valgrind --tool=dhat ./target/release/profiling

# Results saved to dhat.out.*
```

Shows:
- Allocation sizes
- Lifetime of allocations
- Heap usage patterns

## What to Look For

### CPU Profiling
- Functions consuming most CPU time
- Unexpected hot paths
- Recursion depth
- Lock contention

### Memory Profiling
- Allocation hot spots
- Memory leaks
- Excessive allocations
- Peak memory usage

### Cache Profiling
- Cache miss rates
- Data access patterns
- Branch mispredictions

## Optimization Workflow

1. **Profile** - Identify bottlenecks
2. **Hypothesize** - Why is it slow?
3. **Optimize** - Make targeted changes
4. **Benchmark** - Measure improvement
5. **Profile again** - Find next bottleneck

## Example Findings

This demo program shows:
- `inefficient_string_concat`: High allocation rate
- `compute_primes`: CPU-intensive loop
- `create_large_map`: Memory allocation spike
- `hot_path_demo`: Iterator chain optimization

## Tips

1. **Always profile in release mode**
2. **Use realistic workloads**
3. **Focus on hot paths (80/20 rule)**
4. **Profile before and after optimizations**
5. **Keep debug symbols** (`debug = true` in release profile)
