# Prototype Design

The embedded C4-PlantUML diagram shows a client cloning a configured report
template and customizing the independently owned copy.

Flow: construct the baseline once, call `clone`, and change only the fields that
differ for the new report.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Prototype - component view
Person(client, "Client", "Needs a configured report")
System_Boundary(example, "Prototype") {
  Component(prototype, "Report template", "Clone value", "Stores baseline configuration")
  Component(copy, "Report copy", "Owned Rust value", "Receives customization")
}
Rel(client, prototype, "clones")
Rel(prototype, copy, "creates independent copy")
Rel(client, copy, "customizes")
@enduml
```
