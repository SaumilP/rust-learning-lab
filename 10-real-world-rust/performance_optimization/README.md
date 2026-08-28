# Performance and Optimization

Measure before optimizing. Begin with a representative workload, a release
build, and a repeatable baseline. Then change one suspected bottleneck and
measure again.

## Workflow

1. Use `cargo build --release`; debug builds are not performance baselines.
2. Benchmark the smallest meaningful operation and the end-to-end workload.
3. Profile CPU time and allocations to find the actual hot path.
4. Reduce unnecessary allocation, copying, locking, or repeated computation.
5. Confirm correctness and compare the new measurement with the baseline.

Prefer clear algorithms and appropriate data structures before low-level
micro-optimizations. A better complexity class usually matters more than an
instruction-level tweak.
