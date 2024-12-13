extern crate proc_macro;

use std::collections::HashMap;
use std::sync::{RwLock};
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro2::{Span};
use syn::{Ident, Expr, Token, Stmt, Pat, Type, parse_str, ItemFn, Item};
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;

lazy_static!{
    static ref CONTEXT: RwLock<HashMap<String, Class>> = RwLock::new(HashMap::new());
}

#[derive(Clone, Debug)]
struct Class {
    ident: String,
    fields: HashMap<String, String>,
    methods: HashMap<String, String>
}
#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
    let tokens = input.clone();
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser.parse(tokens).unwrap();
    if let Expr::Path(path) = &args[0] {
        if let Some(ident) = path.path.get_ident() {
            let struct_name = ident;
            let mut class = Class {
                ident: struct_name.to_string(),
                fields: HashMap::new(),
                methods: HashMap::new(),
            };
            let mut fields = vec![];
            let mut methods = vec![];
            for elem in &args {
                if let Expr::Block(block) = elem {
                    for stmt in &block.block.stmts {
                        if let Stmt::Local(local) = stmt {
                            if let Pat::Type(pat) = &local.pat {
                                if let Pat::Ident(ident) = &*pat.pat {
                                    if let Type::Path(ty) = &*pat.ty {
                                        class.fields.insert(ident.ident.to_string(), ty.path.get_ident().unwrap().to_string());
                                        fields.push( quote!{
                                            pub #ident: #ty,
                                        })
                                    }
                                }
                            }
                        }
                        if let Stmt::Item(Item::Fn(func)) = stmt {
                            class.methods.insert(func.sig.ident.to_string(), quote!{#func}.to_string());
                            methods.push(
                                quote! { #func }
                            );
                        }
                    }
                };
            }
            CONTEXT.write().unwrap().insert(struct_name.to_string(), class);
            let expanded = quote! {
                struct #struct_name {
                    #(#fields)*
                }
                impl #struct_name {
                    #(#methods)*
                }
            };
            return TokenStream::from(expanded);
        }
    }

    TokenStream::new()
}

#[proc_macro]
pub fn inherit(input: TokenStream) -> TokenStream {
    let tokens = input.clone();
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser.parse(tokens).unwrap();
    if let Expr::Path(path) = &args[0] {
        if let Some(ident) = path.path.get_ident() {
            if let Expr::Path(path) = &args[1] {
                if let Some(parent_ident) = path.path.get_ident() {
                    let parent_class = CONTEXT.read().unwrap().get(&parent_ident.to_string()).unwrap().clone();
                    let struct_name = ident;
                    let mut class = Class {
                        ident: struct_name.to_string(),
                        fields: parent_class.fields.clone(),
                        methods: parent_class.methods.clone()
                    };
                    let mut fields = vec![];
                    let mut methods = vec![];
                    for elem in &args {
                        //println!("{elem:?}");
                        if let Expr::Block(block) = elem {
                            for stmt in &block.block.stmts {
                                if let Stmt::Local(local) = stmt {
                                    if let Pat::Type(pat) = &local.pat {
                                        if let Pat::Ident(ident) = &*pat.pat {
                                            if let Type::Path(ty) = &*pat.ty {
                                                class.fields.insert(ident.ident.to_string(), ty.path.get_ident().unwrap().to_string());
                                                fields.push( quote!{
                                                    pub #ident: #ty,
                                                })
                                            }
                                        }
                                    }
                                }
                                if let Stmt::Item(Item::Fn(func)) = stmt {
                                    class.methods.insert(func.sig.ident.to_string(), quote!{#func}.to_string());
                                    methods.push(
                                        quote! { #func }
                                    );
                                }
                            }
                        };
                    }

                    for (name, ty) in &parent_class.fields {
                        let ident = Ident::new(name, Span::call_site());
                        let ty = parse_str::<Type>(ty).unwrap();
                        fields.push( quote!{
                            pub #ident: #ty,
                        })
                    }
                    for (_, func) in parent_class.methods {
                        let func: ItemFn = parse_str(&func).unwrap();
                        methods.push(quote! {#func} )
                    }

                    CONTEXT.write().unwrap().insert(struct_name.to_string(), class);

                    let expanded = quote! {
                        struct #struct_name {
                            #(#fields)*
                        }
                        impl #struct_name {
                            #(#methods)*
                        }
                    };
                    return TokenStream::from(expanded);
                }
            }
        }
    }
    TokenStream::new()
}