# Strategy Design

The embedded C4-PlantUML diagram shows a context using an interchangeable
algorithm through a trait.

| Component | Responsibility |
|---|---|
| Client | Selects the algorithm |
| Context | Supplies data and invokes the strategy |
| Strategy trait | Defines the algorithm interface |
| Concrete strategies | Implement alternative algorithms |

Use generics when the strategy is fixed at compile time and a trait object when
it must be selected at runtime.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Strategy pattern - component view
Person(client, "Client", "Selects an algorithm")
System_Boundary(example, "Strategy example") {
  Component(context, "Context", "Rust struct", "Uses the selected strategy")
  Component(interface, "Strategy", "Rust trait", "Defines the algorithm interface")
  Component(implementations, "Concrete strategies", "Rust structs", "Implement alternative algorithms")
}
Rel(client, context, "configures and calls")
Rel(context, interface, "uses")
Rel(implementations, interface, "implement")
@enduml
```
