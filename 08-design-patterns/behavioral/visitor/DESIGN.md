# Visitor Design

The embedded C4-PlantUML diagram shows shapes accepting a `ShapeVisitor` and
dispatching to the method for their concrete type.

Flow: iterate over shapes, call `accept`, and let `AreaVisitor` calculate the
correct value for each shape.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Visitor - component view
Person(client, "Client", "Applies an operation")
System_Boundary(example, "Visitor") {
  Component(elements, "Shapes", "Elements", "Accept a visitor")
  Component(visitor, "ShapeVisitor", "Rust trait", "Defines one method per shape")
  Component(area, "AreaVisitor", "Concrete visitor", "Calculates areas")
}
Rel(client, elements, "iterates")
Rel(elements, visitor, "dispatch to")
Rel(area, visitor, "implements")
@enduml
```
