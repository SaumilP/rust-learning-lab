use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type};

/// Derive macro that generates a builder pattern implementation
///
/// # Example
///
/// ```
/// #[derive(Builder)]
/// struct User {
///     id: u32,
///     name: String,
///     email: String,
///     #[builder(optional)]
///     age: Option<u32>,
/// }
///
/// let user = User::builder()
///     .id(1)
///     .name("Alice".to_string())
///     .email("alice@example.com".to_string())
///     .build()
///     .unwrap();
/// ```
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let builder_name = format_ident!("{}Builder", name);

    let fields = match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => {
                return syn::Error::new_spanned(ast, "Builder only supports named fields")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(ast, "Builder only supports structs")
                .to_compile_error()
                .into();
        }
    };

    // Determine which fields are optional
    let field_info: Vec<_> = fields
        .iter()
        .map(|f| {
            let is_optional = f.attrs.iter().any(|attr| {
                attr.path().is_ident("builder")
                    && attr
                        .parse_args::<syn::Ident>()
                        .map(|ident| ident == "optional")
                        .unwrap_or(false)
            });

            let is_option_type = if let Type::Path(type_path) = &f.ty {
                type_path
                    .path
                    .segments
                    .last()
                    .map(|seg| seg.ident == "Option")
                    .unwrap_or(false)
            } else {
                false
            };

            (f, is_optional || is_option_type)
        })
        .collect();

    // Generate builder fields (all Option<T>)
    let builder_fields = field_info.iter().map(|(f, is_optional)| {
        let name = &f.ident;
        let ty = &f.ty;

        if *is_optional {
            quote! { #name: #ty }
        } else {
            quote! { #name: Option<#ty> }
        }
    });

    // Generate setter methods
    let setters = field_info.iter().map(|(f, is_optional)| {
        let name = &f.ident;
        let ty = &f.ty;

        if *is_optional {
            // For optional fields, accept the inner type
            if let Type::Path(type_path) = ty {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "Option" {
                        // Extract T from Option<T>
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                                return quote! {
                                    pub fn #name(mut self, #name: #inner_ty) -> Self {
                                        self.#name = Some(#name);
                                        self
                                    }
                                };
                            }
                        }
                    }
                }
            }
            // Fallback for non-Option optional fields
            quote! {
                pub fn #name(mut self, #name: #ty) -> Self {
                    self.#name = #name;
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(mut self, #name: #ty) -> Self {
                    self.#name = Some(#name);
                    self
                }
            }
        }
    });

    // Generate build method field initialization
    let build_fields = field_info.iter().map(|(f, is_optional)| {
        let name = &f.ident;

        if *is_optional {
            quote! {
                #name: self.#name
            }
        } else {
            quote! {
                #name: self.#name.ok_or_else(|| format!("Field '{}' is required", stringify!(#name)))?
            }
        }
    });

    // Generate builder field initialization
    let builder_init_fields = field_info.iter().map(|(f, _)| {
        let name = &f.ident;
        quote! { #name: None }
    });

    let gen = quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_init_fields,)*
                }
            }
        }

        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            #(#setters)*

            pub fn build(self) -> Result<#name, String> {
                Ok(#name {
                    #(#build_fields,)*
                })
            }
        }
    };

    gen.into()
}
