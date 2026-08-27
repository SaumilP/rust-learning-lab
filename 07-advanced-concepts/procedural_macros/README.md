# Section 17: Procedural Macros

## Overview

Master procedural macros in Rust - write code that writes code. Learn to create custom derive macros, attribute macros, and function-like macros to reduce boilerplate and implement domain-specific languages.

## Why Procedural Macros?

✅ **Metaprogramming** - Generate code at compile time <br />
✅ **Zero Runtime Cost** - All expansion happens during compilation <br />
✅ **Type Safety** - Full access to Rust's type system <br />
✅ **Code Reduction** - Eliminate repetitive boilerplate <br />
✅ **Domain-Specific Languages** - Create custom syntax

## What You'll Learn

1. **Derive Macros** - Auto-implement traits
2. **Attribute Macros** - Modify items with attributes
3. **Function-like Macros** - Create custom macro invocations
4. **TokenStream Manipulation** - Parse and generate code
5. **syn and quote** - Essential macro crates
6. **Hygiene and Spans** - Proper error reporting
7. **Real-World Patterns** - Production macro techniques

## Section Contents

### 01-derive-macro/
Custom derive macro for auto-implementing traits

### 02-attribute-macro/
Attribute macros for function/struct modification

### 03-builder-pattern/
Derive macro for builder pattern generation

## Prerequisites

- Strong Rust fundamentals
- Understanding of traits and generics
- Basic token parsing concepts
- Familiarity with AST (Abstract Syntax Tree)

## Key Concepts

### Procedural Macro Types

Rust has three types of procedural macros:

1. **Derive Macros**: `#[derive(MyTrait)]`
2. **Attribute Macros**: `#[my_attribute]`
3. **Function-like Macros**: `my_macro!()`

### Derive Macro Example

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl HelloWorld for #name {
            fn hello_world(&self) {
                println!("Hello, world! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}
```

Usage:
```rust
#[derive(HelloWorld)]
struct MyStruct;

fn main() {
    let s = MyStruct;
    s.hello_world(); // Prints: Hello, world! My name is MyStruct!
}
```

### Attribute Macro Example

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_entry_exit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;

    let gen = quote! {
        #fn_vis #fn_sig {
            println!("Entering function: {}", stringify!(#fn_name));
            let result = (|| #fn_block)();
            println!("Exiting function: {}", stringify!(#fn_name));
            result
        }
    };

    gen.into()
}
```

Usage:
```rust
#[log_entry_exit]
fn my_function() {
    println!("Inside function");
}

fn main() {
    my_function();
    // Prints:
    // Entering function: my_function
    // Inside function
    // Exiting function: my_function
}
```

### Function-like Macro Example

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    let gen = quote! {
        fn answer() -> u32 {
            42
        }
    };

    gen.into()
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let query = parse_macro_input!(input as LitStr);
    let query_str = query.value();

    // In a real implementation, you would parse and validate SQL
    let gen = quote! {
        {
            let query = #query_str;
            // Return prepared statement or query object
            query
        }
    };

    gen.into()
}
```

Usage:
```rust
make_answer!();

fn main() {
    let result = answer();
    println!("The answer is: {}", result);

    let query = sql!("SELECT * FROM users WHERE id = ?");
}
```

### The syn Crate - Parsing

```rust
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(FieldNames)]
pub fn field_names_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields = match ast.data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(fields_named) => {
                    let field_names: Vec<_> = fields_named.named
                        .iter()
                        .filter_map(|f| f.ident.as_ref())
                        .collect();

                    quote! {
                        impl #name {
                            pub fn field_names() -> &'static [&'static str] {
                                &[#(stringify!(#field_names)),*]
                            }
                        }
                    }
                }
                _ => panic!("Only named fields are supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    fields.into()
}
```

Usage:
```rust
#[derive(FieldNames)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() {
    let fields = User::field_names();
    println!("Fields: {:?}", fields); // ["id", "name", "email"]
}
```

### The quote Crate - Code Generation

```rust
use quote::{quote, format_ident};

// Basic quoting
let tokens = quote! {
    struct Point {
        x: i32,
        y: i32,
    }
};

// Interpolation
let name = format_ident!("MyStruct");
let field_type = quote! { String };

let tokens = quote! {
    struct #name {
        field: #field_type,
    }
};

// Repetition
let field_names = vec!["x", "y", "z"];
let tokens = quote! {
    fn print_fields(&self) {
        #(println!("{}: {:?}", stringify!(#field_names), self.#field_names);)*
    }
};
```

### Builder Pattern Derive Macro

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let builder_name = format_ident!("{}Builder", name);

    let fields = match ast.data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(fields_named) => fields_named.named,
                _ => panic!("Builder only supports named fields"),
            }
        }
        _ => panic!("Builder only supports structs"),
    };

    // Generate builder fields (all Option<T>)
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: Option<#ty> }
    });

    // Generate setter methods
    let setters = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            pub fn #name(mut self, #name: #ty) -> Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    // Generate build method
    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: self.#name.ok_or(concat!("Field '", stringify!(#name), "' is required"))?
        }
    });

    let gen = quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_fields,)*
                }
            }
        }

        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            #(#setters)*

            pub fn build(self) -> Result<#name, &'static str> {
                Ok(#name {
                    #(#build_fields,)*
                })
            }
        }
    };

    gen.into()
}
```

Usage:
```rust
#[derive(Builder)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() {
    let user = User::builder()
        .id(1)
        .name("Alice".to_string())
        .email("alice@example.com".to_string())
        .build()
        .unwrap();
}
```

### Error Handling and Span

```rust
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};
use quote::quote;

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // Validate that it's a struct
    let struct_data = match ast.data {
        syn::Data::Struct(s) => s,
        _ => {
            let error = Error::new_spanned(
                ast,
                "MyTrait can only be derived for structs"
            );
            return error.to_compile_error().into();
        }
    };

    // Check for specific attribute
    let has_attr = ast.attrs.iter().any(|attr| {
        attr.path().is_ident("my_attribute")
    });

    if !has_attr {
        let error = Error::new_spanned(
            ast,
            "MyTrait requires #[my_attribute]"
        );
        return error.to_compile_error().into();
    }

    // Generate implementation
    let name = &ast.ident;
    let gen = quote! {
        impl MyTrait for #name {
            // Implementation
        }
    };

    gen.into()
}
```

### Attribute Arguments

```rust
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta, Lit, Meta};
use quote::quote;

#[proc_macro_attribute]
pub fn cache(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input_fn = parse_macro_input!(input as ItemFn);

    // Parse attribute arguments
    let mut ttl = None;
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("ttl") {
                if let Lit::Int(lit) = nv.lit {
                    ttl = Some(lit.base10_parse::<u64>().unwrap());
                }
            }
        }
    }

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;

    let gen = if let Some(ttl_value) = ttl {
        quote! {
            fn #fn_name() {
                println!("Cache TTL: {}", #ttl_value);
                #fn_block
            }
        }
    } else {
        quote! {
            fn #fn_name() {
                println!("No cache TTL specified");
                #fn_block
            }
        }
    };

    gen.into()
}
```

Usage:
```rust
#[cache(ttl = 60)]
fn expensive_operation() {
    // Function body
}
```

### Macro Helper Attributes

```rust
#[proc_macro_derive(MyTrait, attributes(my_helper))]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // Now we can use #[my_helper] on fields
    let fields = match ast.data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(fields_named) => {
                    fields_named.named.iter().map(|f| {
                        let has_helper = f.attrs.iter().any(|attr| {
                            attr.path().is_ident("my_helper")
                        });

                        if has_helper {
                            // Special handling for marked fields
                        }
                        f
                    }).collect()
                }
                _ => vec![],
            }
        }
        _ => vec![],
    };

    // Generate code
    TokenStream::new()
}
```

Usage:
```rust
#[derive(MyTrait)]
struct User {
    #[my_helper]
    id: u32,
    name: String,
}
```

## Advanced Patterns

### Conditional Compilation

```rust
let tokens = if some_condition {
    quote! {
        fn method_a() { }
    }
} else {
    quote! {
        fn method_b() { }
    }
};
```

### Type Analysis

```rust
use syn::{Type, TypePath};

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        path.segments.last()
            .map(|seg| seg.ident == "Option")
            .unwrap_or(false)
    } else {
        false
    }
}
```

### Generic Support

```rust
let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

let gen = quote! {
    impl #impl_generics MyTrait for #name #ty_generics #where_clause {
        fn method(&self) {
            // Implementation
        }
    }
};
```

### Visibility Handling

```rust
use syn::{Visibility, VisPublic};

fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

// Generate methods with same visibility as struct
let vis = &ast.vis;
let gen = quote! {
    #vis fn new_method() { }
};
```

## Project Structure

```
my-macro/
├── Cargo.toml          # Must include [lib] proc-macro = true
├── src/
│   └── lib.rs          # Macro definitions
└── tests/
    └── integration.rs  # Usage examples

my-macro-derive/        # Separate crate for derive macros
├── Cargo.toml
└── src/
    └── lib.rs

my-app/                 # Consumer application
├── Cargo.toml          # Depends on my-macro
└── src/
    └── main.rs
```

### Cargo.toml for Macro Crate

```toml
[package]
name = "my-macro"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["full"] }
quote = "1.0"
proc-macro2 = "1.0"
```

## Testing Macros

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_macro() {
        #[derive(MyTrait)]
        struct TestStruct {
            field: String,
        }

        let instance = TestStruct {
            field: "test".to_string(),
        };

        // Test generated methods
        assert_eq!(instance.my_method(), expected_value);
    }
}
```

### Using trybuild for Error Testing

```toml
[dev-dependencies]
trybuild = "1.0"
```

```rust
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass/*.rs");
    t.compile_fail("tests/ui/fail/*.rs");
}
```

## Common Patterns

### Serde-style Container/Field Attributes

```rust
#[derive(MyDerive)]
#[my_derive(rename_all = "camelCase")]
struct User {
    #[my_derive(skip)]
    internal_id: u32,

    #[my_derive(rename = "userName")]
    name: String,
}
```

### Auto-implementing ToString

```rust
#[proc_macro_derive(Display)]
pub fn derive_display(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };

    gen.into()
}
```

### Enum Variant Iteration

```rust
#[proc_macro_derive(EnumIter)]
pub fn derive_enum_iter(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let variants = match ast.data {
        Data::Enum(data_enum) => {
            data_enum.variants.iter().map(|v| &v.ident).collect::<Vec<_>>()
        }
        _ => panic!("EnumIter only works on enums"),
    };

    let gen = quote! {
        impl #name {
            pub fn iter() -> impl Iterator<Item = Self> {
                [#(#name::#variants),*].iter().copied()
            }
        }
    };

    gen.into()
}
```

## Debugging Macros

### Print Generated Code

```rust
eprintln!("{}", gen.to_string());
```

### cargo expand

```bash
cargo install cargo-expand
cargo expand
```

### Use macro_rules! for Prototyping

```rust
macro_rules! debug_derive {
    ($name:ident) => {
        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, stringify!($name))
            }
        }
    };
}
```

## Performance Considerations

1. **Compile Time**: Macros increase compilation time
2. **Code Bloat**: Excessive macro expansion can increase binary size
3. **Caching**: Use `cargo check` to speed up development
4. **Lazy Evaluation**: Generate only what's needed

## Best Practices

1. **Error Messages**: Provide clear, helpful error messages with proper spans
2. **Documentation**: Document macro behavior thoroughly
3. **Testing**: Comprehensive tests for edge cases
4. **Versioning**: Use semantic versioning carefully
5. **Hygiene**: Avoid name collisions with generated code
6. **Feature Flags**: Use features for optional functionality

## Resources

- [The Rust Reference - Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)
- [syn Documentation](https://docs.rs/syn/)
- [quote Documentation](https://docs.rs/quote/)
- [proc-macro2 Documentation](https://docs.rs/proc-macro2/)
- [Rust Macro Guide](https://danielkeep.github.io/tlborm/book/)

## Next Steps

After completing this section:
- Write your own derive macro
- Create attribute macros for cross-cutting concerns
- Build a DSL with function-like macros
- Contribute to macro-based libraries

---

**Estimated Time**: 12-16 hours
**Difficulty**: ★★★★☆ (Advanced)
**Prerequisites**: Strong Rust, understanding of AST, trait system
