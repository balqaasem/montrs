# montrs-bench

Professional-grade benchmarking utilities for MontRS.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-bench` is a benchmarking suite designed to measure the performance of Rust code, particularly async operations within the MontRS ecosystem. It provides high-resolution timing, statistical analysis, and system profiling.

## 2. What problems it solves
- **Performance Blindness**: Provides clear, statistical evidence (P95, P99, Ops/Sec) for code performance.
- **Environment Variance**: Profiles the system (CPU, RAM, OS) to ensure that benchmarks are reproducible and comparable across different machines.
- **Boilerplate**: Simplifies the setup/teardown logic for complex benchmarks through the `BenchCase` trait.

## 3. What it intentionally does NOT do
- **Flamegraphs/Profiling**: It does not replace low-level profilers like `perf` or `instruments`. It focuses on timing and statistics.
- **Load Testing**: It is designed for micro-benchmarks and component benchmarks, not for distributed load testing of a production server.
- **Automatic Optimization**: It identifies bottlenecks but does not suggest or apply code changes to fix them.

## 4. How it fits into the MontRS system
It is the engine behind the `montrs bench` command. It allows developers to verify that their plates and actions meet performance requirements.

## 5. When a user should reach for this package
- When optimizing a hot path in their application.
- When comparing the performance of different implementations (e.g., two different ORM queries).
- When validating the performance of a new MontRS plate before submission.

## 6. Deeper Documentation
- [Benchmarking Best Practices](../../docs/testing/benchmarking.md)
- [Interpreting Statistics](../../docs/testing/benchmarking.md#statistics)
- [Writing Advanced Benchmarks](../../docs/testing/benchmarking.md#advanced-usage)

## 7. Notes for Agents
- **Native Mode**: The CLI supports `montrs bench --simple <FILE>` which uses this package to run quick, zero-config benchmarks.
- **Output Parsing**: Prefer the JSON export (`--json-output`) for machine-readable performance data.
- **Error Handling**: Look for `BenchError` with `AgentError` metadata if a benchmark fails to initialize or run.
- **Context Awareness**: Always consider the system profile (CPU/RAM) included in the report when evaluating performance numbers.
