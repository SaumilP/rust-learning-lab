use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, AttributeArgs, NestedMeta, Meta, Lit};

/// Attribute macro that logs function entry and exit
///
/// # Example
///
/// ```
/// #[log_entry_exit]
/// fn my_function() {
///     println!("Doing work");
/// }
/// ```
#[proc_macro_attribute]
pub fn log_entry_exit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;

    let gen = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            println!("→ Entering: {}", stringify!(#fn_name));
            let __result = (|| #fn_block)();
            println!("← Exiting: {}", stringify!(#fn_name));
            __result
        }
    };

    gen.into()
}

/// Attribute macro that times function execution
///
/// # Example
///
/// ```
/// #[time_execution]
/// fn expensive_function() {
///     // Do work
/// }
/// ```
#[proc_macro_attribute]
pub fn time_execution(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;

    let gen = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            let __start = std::time::Instant::now();
            let __result = (|| #fn_block)();
            let __duration = __start.elapsed();
            println!("⏱️  {} took: {:?}", stringify!(#fn_name), __duration);
            __result
        }
    };

    gen.into()
}

/// Attribute macro with arguments for retry logic
///
/// # Example
///
/// ```
/// #[retry(times = 3, delay_ms = 100)]
/// fn flaky_operation() -> Result<(), Error> {
///     // Might fail
/// }
/// ```
#[proc_macro_attribute]
pub fn retry(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);

    let mut times = 3;
    let mut delay_ms = 100;

    // Parse arguments
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("times") {
                if let Lit::Int(lit) = nv.lit {
                    times = lit.base10_parse::<u32>().unwrap();
                }
            } else if nv.path.is_ident("delay_ms") {
                if let Lit::Int(lit) = nv.lit {
                    delay_ms = lit.base10_parse::<u64>().unwrap();
                }
            }
        }
    }

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;

    let gen = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            let mut __attempt = 0;
            loop {
                __attempt += 1;
                let __result = (|| #fn_block)();

                match &__result {
                    Ok(_) => {
                        if __attempt > 1 {
                            println!("✅ {} succeeded on attempt {}", stringify!(#fn_name), __attempt);
                        }
                        return __result;
                    }
                    Err(e) if __attempt < #times => {
                        println!("⚠️  {} failed (attempt {}): {:?}, retrying...",
                                stringify!(#fn_name), __attempt, e);
                        std::thread::sleep(std::time::Duration::from_millis(#delay_ms));
                    }
                    Err(_) => {
                        println!("❌ {} failed after {} attempts", stringify!(#fn_name), __attempt);
                        return __result;
                    }
                }
            }
        }
    };

    gen.into()
}

/// Attribute macro for deprecation warnings
///
/// # Example
///
/// ```
/// #[deprecated_fn(since = "1.2.0", note = "Use new_function instead")]
/// fn old_function() {
///     // Old implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn deprecated_fn(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);

    let mut since = String::from("unknown");
    let mut note = String::from("This function is deprecated");

    // Parse arguments
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("since") {
                if let Lit::Str(lit) = nv.lit {
                    since = lit.value();
                }
            } else if nv.path.is_ident("note") {
                if let Lit::Str(lit) = nv.lit {
                    note = lit.value();
                }
            }
        }
    }

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;

    let gen = quote! {
        #[deprecated(since = #since, note = #note)]
        #fn_vis #fn_sig {
            #fn_block
        }
    };

    gen.into()
}
