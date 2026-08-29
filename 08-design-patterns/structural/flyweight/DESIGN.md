# Flyweight Design

The embedded C4-PlantUML diagram separates per-glyph context from the shared
`TextStyle`. `Rc` lets many glyphs refer to one immutable style allocation.

Flow: create the style once, clone the `Rc` for each glyph, and keep character
and position on the glyph itself.

## C4-PlantUML

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml
title Flyweight - component view
Person(client, "Text renderer", "Creates glyphs")
System_Boundary(example, "Flyweight") {
  Component(glyph, "Glyph", "Context object", "Stores character and position")
  Component(style, "TextStyle", "Shared flyweight", "Stores font and size")
  Component(owner, "Rc", "Shared ownership", "Reuses one style allocation")
}
Rel(client, glyph, "creates")
Rel(glyph, owner, "holds")
Rel(owner, style, "shares")
@enduml
```
