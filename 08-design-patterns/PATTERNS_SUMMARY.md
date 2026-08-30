# Pattern Selection Guide

Use this page after reading the individual examples. It is a reminder of the
design pressure each pattern addresses, not a checklist for application code.

| Pattern | Consider it when | Rust implementation shown |
|---|---|---|
| Abstract Factory | Related products must be created as one family | A factory trait returning several product traits |
| Builder | Construction has several named options or steps | Consuming builder methods returning `Self` |
| Factory | Creation varies but use follows one interface | Enums, traits, and factory functions |
| Factory Method | A shared workflow varies its creation step | A default trait method calling a required factory method |
| Prototype | New values start from an existing configuration | `Clone` followed by owned customization |
| Singleton | One process-wide identity is genuinely required | Lazy initialization and synchronization |
| Adapter | Existing and required interfaces do not match | A wrapper implementing the target trait |
| Bridge | Abstraction and implementation vary independently | A generic abstraction delegating through a trait |
| Composite | Leaves and groups need the same operation | A recursive enum representing a tree |
| Decorator | Independent behaviours need flexible composition | Generic wrappers sharing one trait |
| Facade | One use case coordinates several subsystems | A small orchestration API returning `Result` |
| Flyweight | Many objects repeat large immutable data | Shared ownership through `Rc` or `Arc` |
| Proxy | Access needs caching, authorization, or lazy loading | A wrapper implementing the subject trait |
| Chain of Responsibility | Ordered handlers may accept or pass a request | A sequence of handler trait objects |
| Command | Actions need to be stored, queued, or undone | A trait representing executable work |
| Iterator | A sequence should hide how values are produced | The standard `Iterator` trait |
| Mediator | Components have too many pairwise dependencies | A coordinator that owns routing rules |
| Memento | State needs snapshots or rollback | An owned private snapshot value |
| Observer | Several consumers react to one event | Callbacks, trait objects, or channels |
| State | Operations and transitions depend on current state | An enum with guarded transition methods |
| Strategy | An algorithm must be replaceable | Generic or dynamic trait dispatch |
| Template Method | A workflow is fixed but selected steps vary | A default trait method with required hooks |
| Visitor | Operations change more often than element types | Double dispatch through visitor methods |

## Prefer simpler Rust constructs when possible

- Use a function instead of a command when an action never needs to be stored.
- Use `match` on an enum instead of state objects for a small closed state set.
- Pass a dependency as an argument instead of making it a singleton.
- Use a constructor instead of a builder when there are few parameters.
- Call a subsystem directly instead of adding a facade for a single operation.

## Review questions

For any proposed pattern, ask:

1. What concrete change or duplication is making the current design difficult?
2. Which types own the data?
3. Is runtime polymorphism required, or will generics and enums be clearer?
4. How are failures represented?
5. Can a new engineer follow the control flow without knowing the pattern name?

The [module README](README.md) links to the runnable code and C4-PlantUML design
for every pattern.

For a per-pattern list of counter-signals, read [WHEN_NOT_TO_USE.md](WHEN_NOT_TO_USE.md). The [RUST_NATIVE_PATTERNS.md](RUST_NATIVE_PATTERNS.md) guide covers designs that are often clearer than a direct Gang of Four translation.
