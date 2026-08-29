# Builder Design

The embedded C4-PlantUML diagram shows the client configuring a builder and
the builder returning a complete product.

| Component | Responsibility |
|---|---|
| Client | Supplies construction options |
| `GreetBuilder` | Collects values and validates construction |
| `Greeter` | Represents the completed value |

Flow: the client creates the builder, chains setter methods, and calls `build`.
The product does not need to know how its construction was organized.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Builder pattern - component view
Person(client, "Client", "Chooses the configuration")
System_Boundary(example, "Builder example") {
  Component(builder, "GreetBuilder", "Rust struct", "Collects construction options")
  Component(product, "Greeter", "Rust struct", "Completed value")
}
Rel(client, builder, "sets options")
Rel(builder, product, "builds")
@enduml
```
