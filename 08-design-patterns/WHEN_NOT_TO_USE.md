# When Not to Use a Pattern

A pattern earns its complexity only when it removes a real constraint. This guide is deliberately conservative: start with a direct Rust representation, then introduce a pattern when the direct form begins to hide the important change.

| Pattern | Avoid it when | Prefer |
|---|---|---|
| Abstract Factory | Product families are fixed and small. | An enum or constructors on the concrete types. |
| Builder | Construction has only a few required arguments. | A constructor, a struct literal inside the defining crate, or a configuration struct. |
| Factory and Factory Method | Selection is closed and known at compile time. | An enum plus `match`; use a factory for plugins or runtime choice. |
| Prototype | Copying configuration is cheaper and clearer than preserving identity. | `Clone`, a constructor, or an explicit configuration value. |
| Singleton | A caller can supply the dependency or different tests need different instances. | Explicit dependency injection through function arguments or a context value. |
| Adapter | The two APIs are both under your control. | Rename or redesign the original interface rather than preserving an accidental boundary. |
| Bridge | Only one side is expected to vary. | A generic type or a concrete field. |
| Composite | A tree has different operations for leaves and branches. | A recursive enum with separate `match` arms. |
| Decorator | The layer count is fixed and each layer needs unrelated configuration. | One concrete type or a builder that assembles the final value. |
| Facade | It merely forwards one call. | Call the subsystem directly. |
| Flyweight | Sharing introduces lifetime, synchronization, or cache invalidation costs larger than the saved allocation. | Own the small value, or intern only measured hot data. |
| Proxy | The wrapper changes neither policy nor observable behaviour. | Use the subject directly. |
| Chain of Responsibility | Every request must follow the same known path. | A straightforward function pipeline or `match`. |
| Command | Work is executed immediately and never stored, retried, or undone. | A function or closure. |
| Iterator | A collection API is simpler and no lazy traversal is needed. | Return a collection or expose a slice. |
| Mediator | A coordinator becomes the only place that understands every component. | Direct collaboration with narrow interfaces, or events with clear ownership. |
| Memento | Snapshots are large, frequent, or cannot be restored safely. | Record domain events, use a transaction, or make an explicit copy at a boundary. |
| Observer | Ordering, failure, or backpressure matters to the producer. | A request/response API, a channel with a stated policy, or an async stream. |
| State | States are few and transitions are local. | An enum and `match`; consider typestate only when compile-time transition guarantees justify it. |
| Strategy | Algorithms do not need runtime replacement. | A generic parameter, enum, or ordinary helper function. |
| Template Method | Inheritance-shaped hooks obscure a short workflow. | Compose functions or use a trait only for a genuine extension point. |
| Visitor | Both operations and element variants change often. | Keep operations near the enum and use `match`; visitor is strongest when variants are stable. |

## A quick decision check

Before adding indirection, write the direct version and name the expected change. If the change is speculative, keep the direct version. If the change is real, choose the smallest abstraction that makes ownership, failure, and control flow easier to see.
