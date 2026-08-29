# Command Design

The embedded C4-PlantUML diagram separates the object that triggers an action
from the object that performs it.

| Component | Responsibility |
|---|---|
| Client | Chooses or creates a command |
| Invoker | Triggers the configured command |
| Command | Represents an executable request |
| Receiver | Performs the domain operation |

Because a command is a value, it can be queued, logged, retried, or paired with an inverse operation for undo.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Command pattern - component view
Person(client, "Client", "Chooses an action")
System_Boundary(example, "Command example") {
  Component(invoker, "Invoker", "Rust struct", "Triggers a command")
  Component(command, "Command", "Rust trait object", "Represents the request")
  Component(receiver, "Receiver", "Rust value", "Performs the work")
}
Rel(client, invoker, "triggers")
Rel(invoker, command, "executes")
Rel(command, receiver, "changes")
@enduml
```
