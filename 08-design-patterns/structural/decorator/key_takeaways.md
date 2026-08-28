# Decorator: Key Takeaways

- Component and decorator implement the same trait.
- A decorator delegates to an inner component and adds one concern.
- Generic decorators can avoid dynamic dispatch when the stack is known at
  compile time.
- `Box<dyn Trait>` is useful when decorators are selected at runtime.
- Too many nested decorators can make construction and debugging harder.
