# Async HTTP Load Balancer

This project demonstrates round-robin backend selection, active health checks,
configuration reloads, request plugins, and asynchronous proxying with Tokio
and Hyper.

## Run locally

1. Start the backend services listed in `config.yaml`.
2. From the workspace root, run `cargo run -p load_balancer`.
3. Send a request to `http://127.0.0.1:3000`.

The configuration watcher reloads modified backend settings, while the health
task periodically removes unavailable backends from selection. This is an
educational implementation; production load balancers also require TLS,
timeouts, observability, bounded bodies, and careful retry policies.
