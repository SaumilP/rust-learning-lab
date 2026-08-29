# Facade Design

The embedded C4-PlantUML diagram shows a checkout facade coordinating three
subsystems.

| Component | Responsibility |
|---|---|
| `CheckoutFacade` | Provides the complete checkout operation |
| `Inventory` | Reserves the requested item |
| `PaymentGateway` | Charges the order amount |
| `Shipping` | Produces the shipping label |

Flow: the client calls `checkout`; the facade invokes each subsystem in order
and translates the outcome into one `Result`.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Facade pattern - component view
Person(client, "Client", "Places an order")
System_Boundary(example, "Checkout example") {
  Component(facade, "CheckoutFacade", "Rust struct", "Coordinates checkout")
  Component(inventory, "Inventory", "Subsystem", "Reserves an item")
  Component(payment, "PaymentGateway", "Subsystem", "Charges payment")
  Component(shipping, "Shipping", "Subsystem", "Creates a label")
}
Rel(client, facade, "checkout")
Rel(facade, inventory, "reserve")
Rel(facade, payment, "charge")
Rel(facade, shipping, "create label")
@enduml
```
