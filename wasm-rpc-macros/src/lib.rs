#![feature(
    box_patterns,
)]
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse, punctuated::Punctuated, token::Comma, ArgCaptured, Block, FnArg,
    FnDecl, Item, ItemFn, ItemMod, Pat, PatIdent, ReturnType,
    Visibility,
};

#[proc_macro_attribute]
pub fn export(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match parse(input) {
        Ok(Item::Fn(f)) => export_fn(&f).into(),
        Ok(Item::Mod(m)) => export_mod(m).into(),
        _ => panic!("export can only be applied to a func or a module"),
    }
}

fn export_mod(m: ItemMod) -> TokenStream {
    let funtions: TokenStream = m
        .content
        .unwrap()
        .1
        .iter()
        .map(|item| match item {
            Item::Fn(f) => {
                if is_public(f) {
                    export_fn(f)
                } else {
                    f.into_token_stream()
                }
            }
            _ => item.into_token_stream(),
        })
        .collect();
    quote!(#funtions)
}

fn is_public(f: &ItemFn) -> bool {
    match f {
        ItemFn {
            vis: Visibility::Public(_),
            ..
        } => true,
        _ => false,
    }
}
fn export_fn(f: &ItemFn) -> TokenStream {
    let ItemFn {
        ident,
        decl: box decl,
        vis,
        block,
        ..
    } = f.clone();
    let FnDecl {
        inputs,
        output,
        fn_token,
        ..
    } = decl;
    let pointer_inputs = rewrite_inputs_as_pointers(&inputs);
    let result = dereference_pointers(&output, &inputs, &block);
    let response = wrap_result(&output, &result);

    quote!(
    #[cfg(not(test))]
    #[no_mangle]
    #vis #fn_token #ident (#(#pointer_inputs),*) -> wasm_rpc::Pointer
    {
        wasm_rpc::set_stdio();
        #response
    }
    #[cfg(test)]
    #f
    )
}

fn rewrite_inputs_as_pointers(inputs: &Punctuated<FnArg, Comma>) -> Vec<TokenStream> {
    inputs
        .into_iter()
        .map(|input| match input {
            FnArg::Captured(ArgCaptured { pat, .. }) => quote!(#pat: wasm_rpc::Pointer),
            input => quote!(#input),
        })
        .collect()
}
/// Take all the arguments that will be passed in as pointers and dereference them
/// The final output looks like this
///
/// |a: String, b: u32| {
///     a.len() - b
/// }(wasm_rpc::abort::AbortResultExt::unwrap_or_abort(wasm_rpc::from_slice(&wasm_rpc::Dereferenceable::as_raw_bytes("test"), wasm_rpc::abort::AbortResultExt::unwrap_or_abort(wasm_rpc::from_slice(&wasm_rpc::Dereferenceable::as_raw_bytes(1))
fn dereference_pointers(
    return_type: &ReturnType,
    inputs: &Punctuated<FnArg, Comma>,
    block: &Box<Block>,
) -> TokenStream {
    let pointers: Vec<TokenStream> = inputs.into_iter().map(&dereference_pointer).collect();
    quote!((|#(#inputs),*|#return_type{#block})(#(#pointers),*))
}

fn dereference_pointer(input: &FnArg) -> TokenStream {
    match input {
        FnArg::Captured(ArgCaptured {
            pat: Pat::Ident(PatIdent { ident, .. }),
            ..
        }) => {
            quote!(wasm_rpc::from_slice(&wasm_rpc::Dereferenceable::as_raw_bytes(&#ident)).unwrap())
        }
        _ => panic!("wasm_rpc parse error"),
    }
}

/// All wasm_rpc exports need to return responses
/// If the result of a function is () return Null
///

fn wrap_result(return_type: &ReturnType, result: &TokenStream) -> TokenStream {
    match return_type.clone() {
        ReturnType::Type(_, _) => {
            quote!(wasm_rpc::Referenceable::as_pointer(&wasm_rpc::to_vec(&#result).unwrap()))
        }
        _ => quote!(
            #result;
            wasm_rpc::Referenceable::as_pointer(&wasm_rpc::Value::Null)
        ),
    }
}
