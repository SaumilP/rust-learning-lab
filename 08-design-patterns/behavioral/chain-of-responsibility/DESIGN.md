# Chain of Responsibility Design

The embedded C4-PlantUML diagram shows authentication, rate limiting, and
application handling as an ordered chain.

Flow: visit handlers in order and stop at the first handler that returns a
response. A handler returning `None` passes the request onward.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Chain of Responsibility - component view
Person(client, "Request client", "Submits a request")
System_Boundary(example, "Handler chain") {
  Component(auth, "Authentication", "Handler", "Rejects missing credentials")
  Component(limit, "RateLimit", "Handler", "Rejects blocked requests")
  Component(app, "Application", "Handler", "Produces the final response")
}
Rel(client, auth, "request")
Rel(auth, limit, "passes when unmatched")
Rel(limit, app, "passes when unmatched")
@enduml
```
