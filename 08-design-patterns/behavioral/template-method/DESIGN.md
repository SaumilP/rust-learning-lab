# Template Method Design

The embedded C4-PlantUML diagram shows a default `run` method defining the
pipeline while CSV and line implementations provide the parsing step.

Flow: parse with the selected implementation, apply the shared validation step,
and return the valid values.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Template Method - component view
Person(client, "Client", "Runs a pipeline")
System_Boundary(example, "Template Method") {
  Component(template, "DataPipeline::run", "Default trait method", "Defines parse and validate sequence")
  Component(csv, "CsvPipeline", "Concrete implementation", "Supplies CSV parsing")
  Component(lines, "LinePipeline", "Concrete implementation", "Supplies line parsing")
}
Rel(client, template, "runs")
Rel(template, csv, "calls custom step")
Rel(template, lines, "calls custom step")
@enduml
```
