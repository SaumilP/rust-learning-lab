# Memento Design

The embedded C4-PlantUML diagram shows the editor creating and consuming its
own snapshot while a caretaker only stores the opaque memento.

Flow: save the editor state, make changes, and pass the saved memento back to
`restore` when rollback is required.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Memento - component view
Person(client, "Caretaker", "Requests save and restore")
System_Boundary(example, "Memento") {
  Component(originator, "Editor", "Originator", "Owns editable state")
  Component(snapshot, "EditorMemento", "Memento", "Stores a private snapshot")
}
Rel(client, originator, "save or restore")
Rel(originator, snapshot, "creates and consumes")
Rel(client, snapshot, "stores without inspecting")
@enduml
```
