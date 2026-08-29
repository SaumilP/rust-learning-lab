# Observer Design

The embedded C4-PlantUML diagram shows a publisher distributing one event
to independent subscribers.

| Component | Responsibility |
|---|---|
| Event source | Causes a state change or event |
| Publisher | Maintains subscriptions or senders |
| Observers | React independently to the event |

Callbacks work for synchronous notification. Channels are often a better Rust
choice when observers run concurrently or own their receiving state.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Observer pattern - component view
Person(source, "Event source", "Changes application state")
System_Boundary(example, "Observer example") {
  Component(publisher, "Publisher", "Rust struct", "Stores or connects subscribers")
  Component(observer_a, "Observer A", "Callback, trait, or channel", "Handles the event")
  Component(observer_b, "Observer B", "Callback, trait, or channel", "Handles the event")
}
Rel(source, publisher, "publishes event")
Rel(publisher, observer_a, "notifies")
Rel(publisher, observer_b, "notifies")
@enduml
```
