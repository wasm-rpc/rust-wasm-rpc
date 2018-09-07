#![feature(type_ascription)]
#![feature(box_patterns)]
#![feature(proc_macro_quote)]
#![feature(rustc_private)]
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

#[proc_macro_attribute]
pub fn export(_args: TokenStream, input: TokenStream) -> TokenStream {
    let module: syn::ItemMod = syn::parse(input.clone()).unwrap();
    let module_name = module.clone().ident;
    let iterate_content_for_module = |item| iterate_content_for(module_name.clone(), item);
    let content: Vec<syn::Item> = module
        .clone()
        .content
        .unwrap()
        .1
        .into_iter()
        .map(iterate_content_for_module)
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    quote!(
        use wasm_rpc::{Dereferenceable, Pointer, Responsable};
        mod #module_name;
        #(#content)*
    ).into()
}

fn iterate_content_for(module_name: Ident, item: syn::Item) -> Option<syn::Item> {
    match item.clone() {
        syn::Item::Fn(mut f) => {
            match f.clone().vis {
                syn::Visibility::Public(_) => {
                    f = replace_with_export(module_name, f);
                    Some(syn::Item::Fn(f))
                },
                _ => {
                    None
                }
            }
        }
        _ => None,
    }
}

fn replace_with_export(module_name: Ident, f: syn::ItemFn) -> syn::ItemFn {
    let syn::ItemFn {
        ident,
        decl: box decl,
        vis,
        ..
    } = f.clone();
    let syn::FnDecl {
        inputs,
        _output,
        fn_token,
        ..
    } = decl;

    let new_inputs = inputs
        .clone()
        .into_iter()
        .map(|input| match input {
            syn::FnArg::Captured(syn::ArgCaptured {
                pat: syn::Pat::Ident(syn::PatIdent { ident, .. }),
                ..
            }) => {
                let new_input_name_ident =
                    Ident::new(&format!("_{}_pointer", ident), Span::call_site());
                quote!(#new_input_name_ident: Pointer)
            }
            _ => quote!(input),
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let new_input_names = inputs
        .clone()
        .into_iter()
        .map(|input| match input {
            syn::FnArg::Captured(syn::ArgCaptured {
                pat: syn::Pat::Ident(syn::PatIdent { ident, .. }),
                ..
            }) => {
                let new_input_name_ident =
                    Ident::new(&format!("{}", ident), Span::call_site());
                quote!(#new_input_name_ident)
            }
            _ => quote!(input),
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let type_conversions = inputs
        .clone()
        .into_iter()
        .map(|input| match input.clone() {
            syn::FnArg::Captured(syn::ArgCaptured {
                pat: syn::Pat::Ident(syn::PatIdent { ident, .. }),
                ty,
                ..
            }) => {
                let new_input_name_ident =
                    Ident::new(&format!("_{}_pointer", ident), Span::call_site());
                match ty {
                    syn::Type::Path(syn::TypePath { path, .. }) => {
                        match quote!(#path).to_string().as_ref() {
                            "Vec < u8 >" => quote!(let #ident = #new_input_name_ident.to_bytes();),
                            "u64" => quote!(let #ident: u64 = #new_input_name_ident.to_int();),
                            "u32" => quote!(let #ident: u32 = #new_input_name_ident.to_int() as u32;),
                            "u16" => quote!(let #ident: u16 = #new_input_name_ident.to_int() as u16;),
                            "u8" => quote!(let #ident: u8 = #new_input_name_ident.to_int() as u8;),
                            _ => quote!(),
                        }
                    }
                    _ => quote!(),
                }
            }
            _ => quote!(),
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let new_fn_quote: TokenStream = quote!{
        #[no_mangle]
        #vis #fn_token #ident (#(#new_inputs),*) -> Pointer
        {
            #(#type_conversions);*
            #module_name::#ident(#(#new_input_names),*).to_response()
        }
    }.into();
    syn::parse(new_fn_quote.into()).unwrap()
}
