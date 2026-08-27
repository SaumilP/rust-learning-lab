use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

/// Derive macro to implement a HelloWorld trait
///
/// # Example
///
/// ```
/// use my_derive::HelloWorld;
///
/// #[derive(HelloWorld)]
/// struct MyStruct;
///
/// fn main() {
///     let s = MyStruct;
///     s.hello_world();
/// }
/// ```
#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl HelloWorld for #name {
            fn hello_world(&self) {
                println!("Hello, world! I am {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}

/// Derive macro to generate field_names() method
///
/// Only works on structs with named fields
#[proc_macro_derive(FieldNames)]
pub fn field_names_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields = match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => {
                let field_names: Vec<_> = fields_named
                    .named
                    .iter()
                    .filter_map(|f| f.ident.as_ref())
                    .collect();

                quote! {
                    impl #name {
                        pub fn field_names() -> &'static [&'static str] {
                            &[#(stringify!(#field_names)),*]
                        }

                        pub fn field_count() -> usize {
                            #(stringify!(#field_names),)* .len()
                        }
                    }
                }
            }
            _ => {
                return syn::Error::new_spanned(
                    ast,
                    "FieldNames only supports structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(ast, "FieldNames only supports structs")
                .to_compile_error()
                .into();
        }
    };

    fields.into()
}

/// Derive macro for enum iteration
///
/// Generates an iter() method that returns an iterator over all variants
#[proc_macro_derive(EnumIter)]
pub fn enum_iter_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let variants = match ast.data {
        Data::Enum(data_enum) => {
            let variant_idents: Vec<_> = data_enum
                .variants
                .iter()
                .filter(|v| v.fields == Fields::Unit)
                .map(|v| &v.ident)
                .collect();

            if variant_idents.is_empty() {
                return syn::Error::new_spanned(
                    ast,
                    "EnumIter requires at least one unit variant",
                )
                .to_compile_error()
                .into();
            }

            let count = variant_idents.len();

            quote! {
                impl #name {
                    pub fn iter() -> impl Iterator<Item = Self> {
                        const VARIANTS: [#name; #count] = [
                            #(#name::#variant_idents),*
                        ];
                        VARIANTS.into_iter()
                    }

                    pub fn variant_count() -> usize {
                        #count
                    }
                }
            }
        }
        _ => {
            return syn::Error::new_spanned(ast, "EnumIter only works on enums")
                .to_compile_error()
                .into();
        }
    };

    variants.into()
}

/// Derive macro to implement Default with custom values
///
/// Use #[default_value = "..."] on fields
#[proc_macro_derive(CustomDefault, attributes(default_value))]
pub fn custom_default_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields = match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => {
                let field_inits = fields_named.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();

                    // Check for default_value attribute
                    let default_value = f.attrs.iter().find_map(|attr| {
                        if attr.path().is_ident("default_value") {
                            attr.parse_args::<syn::LitStr>().ok()
                        } else {
                            None
                        }
                    });

                    if let Some(value) = default_value {
                        let value_str = value.value();
                        let tokens: proc_macro2::TokenStream = value_str.parse().unwrap();
                        quote! { #field_name: #tokens }
                    } else {
                        quote! { #field_name: Default::default() }
                    }
                });

                quote! {
                    impl Default for #name {
                        fn default() -> Self {
                            Self {
                                #(#field_inits),*
                            }
                        }
                    }
                }
            }
            _ => {
                return syn::Error::new_spanned(
                    ast,
                    "CustomDefault only supports structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(ast, "CustomDefault only supports structs")
                .to_compile_error()
                .into();
        }
    };

    fields.into()
}
