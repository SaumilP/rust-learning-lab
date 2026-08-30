# Reproducible Benchmarking Protocol

The benchmarks in this directory compare alternative implementations; they do not establish a universal ranking. Treat every result as a measurement of this machine, compiler, dependency set, and input distribution.

## Run a baseline

Run from this directory with a release toolchain. Close unnecessary workloads where practical, record the command and machine details, and use the same command for the comparison.

```bash
cargo bench --bench algorithms -- --save-baseline before-change
cargo bench --bench data_structures -- --save-baseline before-change
```

After a focused implementation change, compare it to that baseline.

```bash
cargo bench --bench algorithms -- --baseline before-change
cargo bench --bench data_structures -- --baseline before-change
```

Criterion writes the HTML report below `target/criterion/`. Its estimates and change reports are the evidence to inspect; do not copy a single timing into documentation as a general performance claim.

## Make a useful comparison

Keep the operation and input data identical between candidates. Include allocation and setup only when they are part of the operation a caller pays for. The existing sorting benchmarks intentionally clone their input inside each iteration because producing a mutable unsorted input is part of sorting work; map lookup benchmarks build their maps outside the timed loop because they isolate lookup work.

Use `black_box` around inputs and outputs when a benchmark could otherwise be optimized away. Check that each candidate produces the same result before trusting a timing. Benchmark enough input sizes to expose changes in algorithmic behavior, then profile the real application before choosing an optimization.

## Record a result

For a change worth keeping, capture the Rust version, target triple, CPU/operating-system context, command, benchmark group, input sizes, and Criterion change report. State the decision the measurement supports and the trade-off it does not answer, such as memory use, ordering guarantees, or readability.

## Boundaries

These examples are microbenchmarks. They are not a substitute for load testing an API, measuring tail latency, or collecting production telemetry. Re-run them after toolchain upgrades or a meaningful hardware change.
