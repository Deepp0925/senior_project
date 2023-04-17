extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

/// A macros that converts enum  to an event emitter that can be used to emit events
/// and listen to them
/// # Example
/// ```no_run
/// use event_emitter::EventEmitter;
/// use std::collections::HashMap;
/// use std::hash::Hash;
///
/// #[derive(EventEmitter)]
/// pub enum MyType {
///     A,
///     B(u32),
///     C(usize, String),
///     D(String),
/// }
///
///
/// ```
/// where T1, T2, T3, T4 are concrete types that implement Clone, Debug, PartialEq, Eq, Hash
/// This will generate the following code:
/// ```no_run
/// #[derive(Debug, PartialEq, Eq, Hash)]
/// pub enum MyType {
///     A,
///     B(u32),
///     C(usize, String),
///     D(String),
/// }
///
/// #[derive(Debug, PartialEq, Eq, Hash)]
/// pub enum OnMyType {
///     A,
///     B,
///     C,
///     D,
/// }
/// pub enum MyTypeHandler {
///     A(fn()),
///     B(fn(&u32)),
///     C(fn(&usize, &String)),
///     D(fn(&String)),
/// }
/// impl From<&MyType> for OnMyType {
///     fn from(handler: &MyType) -> Self {
///         match handler {
///             MyType::A => OnMyType::A,
///             MyType::B(..) => OnMyType::B,
///             MyType::C(..) => OnMyType::C,
///             MyType::D(..) => OnMyType::D,
///         }
///     }
/// }
///
/// impl MyTypeHandler {
///     fn call(&self, event: &MyType) {
///         match event {
///             MyType::A => match self {
///                 MyTypeHandler::A(handler) => handler(),
///                 _ => panic!("wrong handler"),
///             },
///             MyType::B(t1) => match self {
///                 MyTypeHandler::B(handler) => handler(t1),
///                 _ => panic!("wrong handler"),
///             },
///             MyType::C(t2, t3) => match self {
///                 MyTypeHandler::C(handler) => handler(t2, t3),
///                 _ => panic!("wrong handler"),
///             },
///             MyType::D(t4) => match self {
///                 MyTypeHandler::D(handler) => handler(t4),
///                 _ => panic!("wrong handler"),
///             },
///         }
///     }
/// }
///
/// pub struct MyTypeEmitter {
///     handlers: HashMap<OnMyType, SmallVec<[MyTypeHandler; 5]>>,
/// }
///
/// impl MyTypeEmitter {
///     pub fn new() -> Self {
///         Self {
///             handlers: HashMap::new(),
///         }
///     }
///
///     pub fn on(&mut self, event: OnMyType, handler: MyTypeHandler) -> usize {
///         let handlers = self.handlers.entry(event).or_insert_with(SmallVec::new);
///         handlers.push(handler);
///         handlers.len() - 1
///     }
///
///     pub fn emit(&self, event: MyType) {
///         let evt = OnMyType::from(&event);
///         if let Some(handlers) = self.handlers.get(&evt) {
///             for handler in handlers {
///                 handler.call(&event);
///             }
///         }
///     }
///
///     pub fn remove(&mut self, event: OnMyType, id: usize) {
///         if let Some(handlers) = self.handlers.get_mut(&event) {
///             handlers.remove(id);
///         }
///     }
/// }
/// ```
/// # Note
/// The generated code will panic if the wrong handler is called.
/// This is done to avoid silent errors. Currently, there is no way to add a proper type definition
/// so that the compiler can check the handler type.
/// # Limitations
/// - The enum must have at least one variant
#[proc_macro_derive(EventEmitter)]
pub fn event_emitter(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input);
    impl_event_emitter(&ast)
}

fn impl_event_emitter(ast: &syn::DeriveInput) -> TokenStream {
    let vis = &ast.vis;
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("EventEmitter can only be derived for enums"),
    };

    let on_variants = variants.iter().map(|v| {
        let variant_name = &v.ident;
        quote!(#variant_name)
    });
    let on_name = format_ident!("On{}", name);
    let on_enum = quote! {
        #[derive(Debug, PartialEq, Eq, Hash)]
        #vis enum #on_name {
            #(#on_variants),*
        }
    };

    // generate the enum handler
    let handlers = variants.iter().map(|v| {
        let variant_name = &v.ident;
        let token_stream = match &v.fields {
            syn::Fields::Unnamed(fields) => {
                let types = fields.unnamed.iter().map(|f| {
                    let ty = &f.ty;
                    quote!(&#ty)
                });

                return quote!(#variant_name(fn(#(#types),*)));
            }
            syn::Fields::Unit => quote!(#variant_name(fn())),
            _ => panic!("EventEmitter can only be derived for enums with unnamed fields"),
        };

        token_stream
    });
    let handler_name = format_ident!("{}Handler", name);
    let handlers_enum = quote! {
        #vis enum #handler_name {
            #(#handlers),*
        }
    };

    // generate the From impl
    let from_impl = {
        let match_arms = variants.iter().map(|v| {
            let variant_name = &v.ident;

            match &v.fields {
                syn::Fields::Unnamed(_) => {
                    quote!(#name::#variant_name(..) => #on_name::#variant_name)
                }
                syn::Fields::Unit => quote!(#name::#variant_name => #on_name::#variant_name),
                _ => panic!("EventEmitter can only be derived for enums with unnamed fields"),
            }
        });

        quote! {
            impl From<&#name> for #on_name {
                fn from(handler: &#name) -> Self {
                    match handler {
                        #(#match_arms),*
                    }
                }
            }
        }
    };

    let impl_handler = {
        let evt_match_arms = variants.iter().map(|v| {
            let variant_name = &v.ident;
            let evt_match_arm = match &v.fields {
                syn::Fields::Unnamed(fields) => {
                    let mut i: u8 = 0;
                    let params = fields.unnamed.iter().map(|_| {
                        let ident = format_ident!("t{}", i);
                        i += 1;
                        ident
                    });

                    let match_arm = quote! {
                        #handler_name::#variant_name(handler) => handler(#(#params),*)
                    };

                    let mut i: u8 = 0;
                    let params = fields.unnamed.iter().map(|_| {
                        let ident = format_ident!("t{}", i);
                        i += 1;
                        ident
                    });

                    return quote! {
                        #name::#variant_name(#(#params),*) => match self {
                            #match_arm,
                            _ => panic!("wrong handler"),
                        },
                    };
                }
                syn::Fields::Unit => quote! {
                        #name::#variant_name => match self {
                            #handler_name::#variant_name(handler) => handler(),
                            _ => panic!("wrong handler"),
                        },
                },
                _ => panic!("EventEmitter can only be derived for enums with unnamed fields"),
            };

            evt_match_arm
        });

        quote! {
            impl #handler_name {
                fn call(&self, event: &#name) {
                    match event {
                        #(#evt_match_arms)*
                    }
                }
            }
        }
    };

    let emitter_name = format_ident!("{}Emitter", name);
    let emitter_struct = quote! {
        #vis struct #emitter_name {
            handlers: std::collections::HashMap<#on_name, smallvec::SmallVec<[#handler_name; 5]>>
        }
    };

    let impl_emitter = quote! {
        impl #emitter_name {
            #vis fn new() -> Self {
                Self {
                    handlers: std::collections::HashMap::new(),
                }
            }

            #vis fn on(&mut self, event: #on_name, handler: #handler_name) -> usize {
                let handlers = self.handlers.entry(event).or_insert_with(smallvec::SmallVec::new);
                handlers.push(handler);
                handlers.len() - 1
            }

            #vis fn emit(&self, event: #name) {
                let evt = #on_name::from(&event);
                if let Some(handlers) = self.handlers.get(&evt) {
                    for handler in handlers {
                        handler.call(&event);
                    }
                }
            }

            #vis fn remove(&mut self, event: #on_name, id: usize) {
                if let Some(handlers) = self.handlers.get_mut(&event) {
                    handlers.remove(id);
                }
            }
        }
    };

    let expanded = quote! {
        #on_enum

        #handlers_enum

        #from_impl

        #impl_handler

        #emitter_struct

        #impl_emitter
    };

    expanded.into()
}
