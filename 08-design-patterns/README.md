# Design Patterns in Rust

This module presents twenty-three common design patterns as runnable Rust programs. Each
example is intentionally small: it shows the participants, one practical use,
and the Rust feature that makes the pattern work.

Patterns are vocabulary, not requirements. Prefer an ordinary function, enum,
or struct when it solves the problem clearly. Use a named pattern when it makes
a recurring design constraint easier to explain and maintain.

## Pattern index

| Category | Pattern | Application in the example |
|---|---|---|
| Creational | [Abstract Factory](creational/abstract-factory/) | Create matching light or dark UI controls |
| Creational | [Builder](creational/builder/) | Assemble a configured value step by step |
| Creational | [Factory](creational/factory/) | Choose a concrete product behind a shared interface |
| Creational | [Factory Method](creational/factory-method/) | Let an importer choose its parser |
| Creational | [Prototype](creational/prototype/) | Clone and customize a report template |
| Creational | [Singleton](creational/singleton/) | Initialize shared process state once |
| Structural | [Adapter](structural/adapter/) | Convert a legacy Fahrenheit API to Celsius |
| Structural | [Bridge](structural/bridge/) | Control different devices through one remote abstraction |
| Structural | [Composite](structural/composite/) | Treat files and directories through one operation |
| Structural | [Decorator](structural/decorator/) | Compose message formatting behaviours |
| Structural | [Facade](structural/facade/) | Coordinate inventory, payment, and shipping |
| Structural | [Flyweight](structural/flyweight/) | Share immutable text styles between glyphs |
| Structural | [Proxy](structural/proxy/) | Cache access to a database-like service |
| Behavioral | [Chain of Responsibility](behavioral/chain-of-responsibility/) | Process a request through ordered handlers |
| Behavioral | [Command](behavioral/command/) | Represent an action as a value |
| Behavioral | [Iterator](behavioral/iterator/) | Traverse a custom countdown sequence |
| Behavioral | [Mediator](behavioral/mediator/) | Route chat messages through a central room |
| Behavioral | [Memento](behavioral/memento/) | Save and restore editor state |
| Behavioral | [Observer](behavioral/observer/) | Notify independent subscribers of an event |
| Behavioral | [State](behavioral/state/) | Guard an order's state transitions |
| Behavioral | [Strategy](behavioral/strategy/) | Select an algorithm through a trait |
| Behavioral | [Template Method](behavioral/template-method/) | Share a pipeline while varying one step |
| Behavioral | [Visitor](behavioral/visitor/) | Add area calculation across shape types |

Every pattern directory contains:

- `src/main.rs`: runnable Rust code;
- `README.md`: the problem, example, and usage guidance;
- `key_takeaways.md`: a short review;
- `DESIGN.md`: participants, execution flow, and embedded C4-PlantUML source;
- `Makefile`: the same build, run, test, lint, and cleanup interface.

## Run the examples

Run one pattern from this directory:

```bash
make run PATTERN=adapter
```

Run every pattern:

```bash
make run-all
```

Run all repository checks for the module:

```bash
make check
```

You can also work inside one pattern directory:

```bash
cd structural/adapter
make run
make check
```

Generated Cargo output is stored in `target/`. Remove it after a study or
verification session with:

```bash
make clean
```

## Reading the diagrams

Each `DESIGN.md` embeds its C4-PlantUML component source directly in a
`plantuml` code block so GitHub readers do not need to open a separate diagram
file. The diagram shows roles and dependencies, not every Rust type or method.

To render a diagram locally, copy its `plantuml` block into a temporary `.puml`
file and run PlantUML against that file. The Markdown remains the authoritative
source.

## Rust and Java comparison

The same pattern name can lead to a different implementation in Rust. Rust uses
traits, enums, ownership, and composition instead of class inheritance. For a
Java-oriented comparison, see Saumil Patel's
[Java design-pattern examples](https://github.com/SaumilP/design-patterns).

Compare the problem and participants first; do not translate Java classes into
Rust types one for one.

See [PATTERNS_SUMMARY.md](PATTERNS_SUMMARY.md) for a compact selection guide.
