# State Design

The embedded C4-PlantUML diagram shows an order and its guarded transition
methods.

| Component | Responsibility |
|---|---|
| Order service | Requests a state transition |
| `Order` | Owns the current state |
| `OrderState` | Names the possible states |
| Transition methods | Permit valid changes and reject invalid ones |

The example's valid path is `Draft -> Paid -> Shipped`. An attempt to ship a
draft order returns an error and leaves the state unchanged.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title State pattern - component view
Person(client, "Order service", "Requests transitions")
System_Boundary(example, "State example") {
  Component(order, "Order", "Rust struct", "Owns the current state")
  Component(state, "OrderState", "Rust enum", "Draft, Paid, or Shipped")
  Component(rules, "Transition methods", "Rust methods", "Validate pay and ship")
}
Rel(client, order, "pay or ship")
Rel(order, rules, "delegates transition")
Rel(rules, state, "reads and updates")
@enduml
```
