# Project Architecture and Observability

The advanced examples are deliberately small, but their boundaries are the same boundaries a production Rust service needs. Use the Axum example in [modern web APIs](../../07-advanced-concepts/modern_web_apis/02-axum-api/README.md) as the concrete reference: routes compose handlers, `AppState` owns the database pool, middleware surrounds the router, and handlers convert infrastructure failures into HTTP responses.

## Keep dependency direction simple

Start at the edge and move inward: transport code parses HTTP or messages, application code coordinates a use case, domain code owns business rules, and infrastructure code implements databases or external clients. A handler should be thin enough to test its decision separately from Axum. State should contain long-lived, shareable dependencies such as a connection pool or client, not request-local data.

The repository's Axum example currently keeps this structure in one file so the framework concepts stay visible. When it grows, split by responsibility rather than by technical novelty: `routes` wires endpoints, `handlers` performs transport translation, `service` coordinates operations, `model` owns values, and `infrastructure` creates clients and pools. Keep the router composition point obvious; it is where middleware and state ownership can be reviewed together.

## Observability is part of the boundary

`TraceLayer::new_for_http()` in the Axum example creates request spans, while `tracing_subscriber::fmt::init()` installs a local formatter. That is enough to see request activity during development, but logs only become operationally useful when they answer which request failed, which dependency was involved, and how long the operation took.

Add fields that describe the operation rather than sensitive payloads. Good fields include route name, HTTP method, response status, elapsed time, retry count, and a stable request or correlation identifier. Do not record authorization headers, access tokens, passwords, full database URLs, or personal data. Emit a structured error at the point where context is richest, then translate it to the client-facing error once.

```rust
tracing::info!(user_id = id, "creating user record");
tracing::warn!(%request_id, status = 503, "database temporarily unavailable");
```

The example does not yet create request identifiers or metrics. Treat those as the next production increment, not as behavior it already guarantees.

## Health is not readiness

The sample `/api/health` route proves that the process can answer HTTP. A production service normally needs a separate readiness check that verifies only dependencies required to receive traffic, such as a database pool. Keep this check cheap and bounded by a timeout. Liveness should not restart a process merely because a downstream dependency is temporarily unavailable; readiness should remove it from traffic while the dependency is unavailable.

## Practical delivery checklist

- Define a route-level request span and propagate a request identifier through outbound calls.
- Configure log level and output format through environment or deployment configuration, not source edits.
- Add bounded timeouts, retries only where the operation is safe to repeat, and metrics for latency, errors, and saturation.
- Keep database, queue, and HTTP client construction at the composition root so tests can substitute them.
- Give health, readiness, and dependency failures distinct semantics.
- Exercise error paths and verify that logs preserve useful context without leaking secrets.

This guidance describes an architecture direction for the existing examples. It is not a claim that the sample API includes authentication, metrics export, distributed tracing, or a production deployment configuration.
