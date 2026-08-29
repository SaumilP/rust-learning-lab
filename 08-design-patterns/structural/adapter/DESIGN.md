# Adapter Design

The embedded C4-PlantUML diagram shows the boundary between the new client
interface and a legacy temperature API.

| Component | Responsibility |
|---|---|
| Display | Reads temperatures through the target trait |
| `TemperatureSource` | Defines the client's expected unit and method |
| `CelsiusAdapter` | Converts Fahrenheit to Celsius |
| `LegacyThermometer` | Provides the existing Fahrenheit interface |

Flow: the display calls the trait, the adapter reads the legacy value, converts
it, and returns the unit the display expects.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Adapter pattern - component view
Person(client, "Display", "Needs a Celsius temperature")
System_Boundary(example, "Adapter example") {
  Component(target, "TemperatureSource", "Rust trait", "Interface expected by the client")
  Component(adapter, "CelsiusAdapter", "Rust struct", "Converts Fahrenheit to Celsius")
  Component(legacy, "LegacyThermometer", "Existing type", "Returns Fahrenheit")
}
Rel(client, target, "reads")
Rel(adapter, target, "implements")
Rel(adapter, legacy, "wraps and converts")
@enduml
```
