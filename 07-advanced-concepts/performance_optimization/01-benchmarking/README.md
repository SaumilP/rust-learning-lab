# Benchmarking with Criterion

Comprehensive benchmarking examples using criterion for statistical analysis.

Read the [reproducible benchmarking protocol](BENCHMARKING_PROTOCOL.md) before interpreting a result. It explains the commands, fairness rules, and limits of these microbenchmarks.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench fibonacci

# Generate detailed report
cargo bench -- --verbose

# Save baseline
cargo bench -- --save-baseline baseline_name

# Compare against baseline
cargo bench -- --baseline baseline_name
```

## Benchmarks Included

### algorithms.rs
- Fibonacci (recursive vs iterative)
- Sorting algorithms (bubble sort vs std)
- String concatenation strategies
- Sum implementations (loop vs iterator vs fold)

### data_structures.rs
- HashMap vs BTreeMap (insert, lookup)
- HashSet vs BTreeSet (insert, contains)
- Vec operations (with/without capacity)

## What You'll Learn

- Using criterion for statistical benchmarking
- Comparing multiple implementations
- Parametric benchmarks
- Baseline comparisons
- HTML report generation

## Best Practices

1. **Always use `black_box`** - Prevents compiler optimization
2. **Run in release mode** - Benchmarks automatically use release
3. **Multiple iterations** - Criterion handles this automatically
4. **Warm up** - Criterion warms up before measuring
5. **Outlier detection** - Criterion automatically detects outliers

## Viewing Results

After running benchmarks:
```bash
# Open HTML report
open target/criterion/report/index.html
```

The report includes:
- Violin plots
- Performance comparisons
- Statistical analysis
- Regression detection
