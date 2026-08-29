# Proxy Design

The embedded C4-PlantUML diagram shows the client using `UserStore` while
`CachedStore` decides whether to return a cached value or call `Database`.

Flow: check the cache, delegate a miss to the real subject, store the successful
result, and return it through the same trait.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Proxy - component view
Person(client, "Client", "Looks up a user")
System_Boundary(example, "Proxy") {
  Component(subject, "UserStore", "Rust trait", "Common lookup interface")
  Component(proxy, "CachedStore", "Proxy", "Returns cached values")
  Component(real, "Database", "Real subject", "Performs the underlying lookup")
}
Rel(client, subject, "calls")
Rel(proxy, subject, "implements")
Rel(proxy, real, "delegates cache misses")
@enduml
```
