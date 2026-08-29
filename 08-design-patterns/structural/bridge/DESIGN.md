# Bridge Design

The embedded C4-PlantUML diagram shows `Remote` as the abstraction and
`Device` as its implementation boundary. Television and radio can vary without
creating a separate remote hierarchy for each device.

Flow: the client calls the remote, which delegates the device-specific work
through the trait.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Bridge - component view
Person(client, "Client", "Uses a remote")
System_Boundary(example, "Bridge") {
  Component(remote, "Remote", "Abstraction", "Exposes control operations")
  Component(device, "Device", "Implementation trait", "Defines device operations")
  Component(types, "Television and Radio", "Concrete implementations", "Control hardware")
}
Rel(client, remote, "uses")
Rel(remote, device, "delegates to")
Rel(types, device, "implement")
@enduml
```
