# Singleton Design

The embedded C4-PlantUML diagram shows multiple callers reaching the same
lazily initialized process-wide value.

| Component | Responsibility |
|---|---|
| Callers | Request shared configuration or state |
| Global access point | Performs one-time initialization |
| Shared instance | Stores the value and synchronizes mutation when required |

Rust global state must be thread-safe. Prefer passing dependencies explicitly
unless process-wide identity is a real requirement.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Singleton pattern - component view
Person(client_a, "Caller A", "Reads or updates shared state")
Person(client_b, "Caller B", "Reads or updates shared state")
System_Boundary(example, "Singleton example") {
  Component(access, "Global access point", "once_cell::Lazy", "Initializes the value once")
  Component(instance, "Shared instance", "Rust value with locks", "Holds process-wide state")
}
Rel(client_a, access, "accesses")
Rel(client_b, access, "accesses")
Rel(access, instance, "returns the same instance")
@enduml
```
