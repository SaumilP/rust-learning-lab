# Mediator Design

The embedded C4-PlantUML diagram shows senders communicating through a
`ChatRoom` rather than keeping direct references to every recipient.

Flow: the sender provides a message to the room, and the room applies routing
rules to produce deliveries for the other members.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Mediator - component view
Person(sender, "Sender", "Publishes a chat message")
System_Boundary(example, "Mediator") {
  Component(room, "ChatRoom", "Mediator", "Knows members and routing rules")
  Component(member_a, "Member A", "Colleague", "Receives routed messages")
  Component(member_b, "Member B", "Colleague", "Receives routed messages")
}
Rel(sender, room, "broadcast")
Rel(room, member_a, "delivers")
Rel(room, member_b, "delivers")
@enduml
```
