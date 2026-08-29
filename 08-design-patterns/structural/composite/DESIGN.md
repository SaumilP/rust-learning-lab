# Composite Design

The embedded C4-PlantUML diagram shows the `FileNode` enum representing both
leaf files and composite directories.

Flow: `size` returns a file's byte count directly or recursively sums the sizes
of every child in a directory.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Composite - component view
Person(client, "Client", "Requests a node size")
System_Boundary(example, "Composite") {
  Component(component, "FileNode", "Rust enum", "Common file-system operation")
  Component(leaf, "File", "Leaf variant", "Returns its byte count")
  Component(composite, "Directory", "Composite variant", "Sums child sizes")
}
Rel(client, component, "calls size")
Rel(leaf, component, "is a variant of")
Rel(composite, component, "contains child nodes")
@enduml
```
