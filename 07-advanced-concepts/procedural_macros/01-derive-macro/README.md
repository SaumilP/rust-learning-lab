# Derive Macro Examples

Custom derive macros for auto-implementing traits and generating boilerplate code.

## Macros Included

1. **HelloWorld** - Implements HelloWorld trait
2. **FieldNames** - Generates field_names() method for structs
3. **EnumIter** - Creates iter() method for enum variants
4. **CustomDefault** - Implements Default with custom values

## Project Structure

```
01-derive-macro/
├── my-derive/      # The proc-macro crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── example/        # Usage examples
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Running

```bash
cd example
cargo run
```

## What You'll Learn

- Creating derive macros with `#[proc_macro_derive]`
- Parsing structs and enums with `syn`
- Generating code with `quote`
- Using helper attributes
- Error handling in macros
- Proper span usage for error messages

## Key Concepts

### Derive Macro Structure

```rust
#[proc_macro_derive(MacroName)]
pub fn macro_name_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // Parse and generate code
}
```

### Helper Attributes

```rust
#[proc_macro_derive(MacroName, attributes(helper_attr))]
pub fn macro_name_derive(input: TokenStream) -> TokenStream {
    // Can now use #[helper_attr] on fields/variants
}
```
