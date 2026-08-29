# Iterator Design

The embedded C4-PlantUML diagram shows `Countdown` implementing Rust's
standard `Iterator` interface and a consumer using `collect`.

Flow: the consumer repeatedly calls `next` through an adapter until it receives
`None`.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Iterator - component view
Person(client, "Consumer", "Requests sequence items")
System_Boundary(example, "Iterator") {
  Component(source, "Countdown", "Rust struct", "Stores traversal state")
  Component(interface, "Iterator", "Standard trait", "Defines next")
  Component(adapter, "collect", "Iterator adapter", "Consumes the sequence")
}
Rel(source, interface, "implements")
Rel(client, adapter, "calls")
Rel(adapter, interface, "repeatedly calls next")
@enduml
```
