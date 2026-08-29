# Abstract Factory Design

The embedded C4-PlantUML diagram shows a UI client depending on one
`UiFactory`. `LightTheme` and `DarkTheme` each create a button and checkbox that
belong to the same visual family.

Flow: select a concrete factory, request each product through the factory trait,
and use the returned product traits without matching on concrete types.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Abstract Factory - component view
Person(client, "UI client", "Selects one theme")
System_Boundary(example, "Abstract Factory") {
  Component(factory, "UiFactory", "Rust trait", "Creates a product family")
  Component(light, "LightTheme", "Concrete factory", "Creates light controls")
  Component(dark, "DarkTheme", "Concrete factory", "Creates dark controls")
  Component(products, "Button and Checkbox", "Product traits", "Controls used by the client")
}
Rel(client, factory, "renders through")
Rel(light, factory, "implements")
Rel(dark, factory, "implements")
Rel(factory, products, "creates matching family")
@enduml
```
