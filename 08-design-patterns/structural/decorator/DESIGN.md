# Decorator Design

The embedded C4-PlantUML diagram shows two wrappers around a base notifier.

| Component | Responsibility |
|---|---|
| `Notifier` | Defines the common operation |
| `PlainNotifier` | Produces the base message |
| `Urgent` | Adds an urgency prefix |
| `WithSignature` | Adds sender information |

Flow: each decorator adds its concern and delegates to the next component. The
client only needs the `Notifier` interface.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Decorator pattern - component view
Person(client, "Client", "Sends a notification")
System_Boundary(example, "Decorator example") {
  Component(interface, "Notifier", "Rust trait", "Common component interface")
  Component(signature, "WithSignature", "Decorator", "Adds a sender")
  Component(urgent, "Urgent", "Decorator", "Adds an urgency prefix")
  Component(plain, "PlainNotifier", "Component", "Returns the base message")
}
Rel(client, interface, "calls")
Rel(signature, interface, "implements")
Rel(signature, urgent, "wraps")
Rel(urgent, plain, "wraps")
@enduml
```
