# Factory Method Design

The embedded C4-PlantUML diagram shows the `Importer` workflow calling its
`create_parser` customization point. Each importer chooses a parser while the
rest of the import sequence remains shared.

Flow: call `import`, create the selected parser, parse the input, and return the
number of imported values.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Factory Method - component view
Person(client, "Client", "Runs an import")
System_Boundary(example, "Factory Method") {
  Component(creator, "Importer", "Rust trait", "Defines import workflow and factory method")
  Component(concrete, "Concrete importer", "Rust struct", "Chooses a parser")
  Component(product, "Parser", "Rust trait", "Parses the input")
}
Rel(client, creator, "calls import")
Rel(concrete, creator, "implements")
Rel(creator, product, "creates and uses")
@enduml
```
