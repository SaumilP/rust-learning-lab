# Rust-Native Design Patterns

The GoF names remain useful, but Rust has tools that often express the same intent with fewer moving parts. These are not replacements in every case; they are the first alternatives worth checking.

## Model a closed domain with enums

Use an enum when all variants belong to the application and are known together. `match` keeps each case visible, lets the compiler check exhaustiveness, and commonly replaces class hierarchies, simple factories, visitors, and state objects. Add a trait object only when downstream code must introduce new implementations without editing the enum.

## Make invalid values difficult to construct

Use a newtype with a validating constructor for values such as account identifiers, bounded percentages, and parsed addresses. The type carries the invariant instead of asking every caller to remember it. For transitions whose order is central to safety, typestate can encode the permitted next operation in the type parameter; use it sparingly because it makes APIs more advanced.

## Let ownership describe resources

RAII is the default resource-management pattern in Rust. A value acquires a resource, its owner controls the lifetime, and `Drop` releases it when the value leaves scope. Prefer this model over manual `close` or `dispose` protocols. If cleanup can fail, provide an explicit `finish` or `shutdown` method and document what happens when it is skipped.

## Represent fallibility in the type

Use `Result<T, E>` for expected failure and `Option<T>` for meaningful absence. This makes error paths part of the signature and usually removes exception-style control flow. Add context at application boundaries, keep library error types specific enough for recovery, and avoid `unwrap` outside examples or invariants that are clearly explained.

## Prefer static dispatch until runtime choice is required

Generics and `impl Trait` give zero-cost abstraction when one concrete implementation is selected per call site. Use `dyn Trait` when a heterogeneous collection, plugin boundary, or runtime strategy choice is genuinely necessary. This distinction is often the Rust form of Strategy, Bridge, Decorator, and Factory.

## Use iterators and adapters for data flow

Iterator combinators express lazy transformations without a separate pipeline framework. Use `map`, `filter`, `filter_map`, `try_fold`, and `collect` when they reveal the transformation; use a `for` loop when names, branching, or early exits make it easier to read. Iterators commonly replace hand-written visitor and command-like traversal code.

## Choose concurrency boundaries explicitly

`std::sync::mpsc` channels, shared `Arc` ownership, and synchronization primitives solve different problems. Prefer message passing when one owner should serialize changes; prefer `Arc<Mutex<T>>` or `Arc<RwLock<T>>` only when shared mutable state is the actual model. In async code, do not hold a blocking lock across an `.await`; choose async-aware primitives when tasks must wait cooperatively.

## Compose dependencies instead of locating them globally

Construct application services at the edge and pass the narrow capability each component needs. A small trait can make a boundary testable, but a concrete reference is better when there is no alternative implementation. This is explicit composition, not a container pattern, and it keeps configuration and ownership visible.
