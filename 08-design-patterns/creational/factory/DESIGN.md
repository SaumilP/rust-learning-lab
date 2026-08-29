# Factory Design

The embedded C4-PlantUML diagram shows how creation is separated from use.

| Component | Responsibility |
|---|---|
| Client | Requests a product and uses its common interface |
| Factory | Selects and constructs the concrete product |
| Product trait | Defines what clients can do |
| Concrete products | Supply type-specific behaviour |

Flow: the client supplies a kind or configuration, the factory constructs the
matching product, and the client continues through the product interface.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Factory pattern - component view
Person(client, "Client", "Requests a product by kind")
System_Boundary(example, "Factory example") {
  Component(factory, "Factory", "Rust function or struct", "Selects and creates a product")
  Component(interface, "Product trait", "Rust trait", "Interface used by the client")
  Component(products, "Concrete products", "Rust structs", "Provide product-specific behaviour")
}
Rel(client, factory, "requests product")
Rel(factory, products, "constructs")
Rel(products, interface, "implement")
Rel(client, interface, "uses")
@enduml
```
