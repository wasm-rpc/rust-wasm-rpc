#![feature(
    type_ascription,
    box_patterns,
    proc_macro_quote,
    rustc_private,
    proc_macro_hygiene
)]
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse, punctuated::Pair, punctuated::Punctuated, token::Comma, ArgCaptured, Block, FnArg,
    FnDecl, Item, ItemFn, ItemMod, Pat, PatIdent, Path, PathSegment, ReturnType, Type, TypePath,
    Visibility,
};

const PRIMATIVES: &'static [&str] = &["i64", "i32", "i16", "i8", "u64", "u32", "u16", "u8"];

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
/// Take all the arugments that will be passed in as pointers and dereferences them
/// The final output looks like this
///
/// |a: String, b: u32| {
///     a.len() - b
/// }(wasm_rpc::Dereferenceable.to_string("test"), wasm_rpc::Dereferenceable.to_i64(1))
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
            ty: Type::Path(TypePath { path, .. }),
            ..
        }) => {
            if is_primative(path) {
                let dref_fn = dereference_function(path);

                quote!(wasm_rpc::Dereferenceable::#dref_fn(&#ident) as #path)
            } else {
                let dref_fn = dereference_function(path);

                quote!(wasm_rpc::Dereferenceable::#dref_fn(&#ident))
            }
        }
        _ => panic!("wasm_rpc parse error"),
    }
}

fn is_primative(path: &Path) -> bool {
    PRIMATIVES.contains(&path_to_string(path).as_str())
}

fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::Path(TypePath { path, .. }) => path_to_string(path),
        _ => panic!("error parsing type"),
    }
}

fn path_to_string(path: &Path) -> String {
    match path.segments.first() {
        Some(Pair::End(PathSegment { ident, .. })) => ident.to_string(),
        _ => panic!("error parsing type path"),
    }
}

fn dereference_function(path: &Path) -> proc_macro2::Ident {
    if is_primative(path) {
        Ident::new("to_i64", Span::call_site())
    } else {
        match path_to_string(path).as_ref() {
            "BTreeMap" => Ident::new("to_object", Span::call_site()),
            "String" => Ident::new("to_string", Span::call_site()),
            "Vec" => Ident::new("to_bytes", Span::call_site()),
            "Value" => Ident::new("to_value", Span::call_site()),
            path_string => panic!("unsupportd wasm_rpc type: {}", path_string),
        }
    }
}

/// All wasm_rpc exports need to return responses
/// If the function returns a raw response type wrap it with `Ok()`.
/// If the result of a function is () return Ok(Null).
///

fn wrap_result(return_type: &ReturnType, result: &TokenStream) -> TokenStream {
    match return_type.clone() {
        ReturnType::Type(_, box path) => {
            if type_to_string(&path).starts_with("Result") {
                quote!(wasm_rpc::Responsable::to_response((#result)))
            } else {
                quote!(wasm_rpc::Responsable::to_response(Ok(#result)))
            }
        }
        ReturnType::Default => quote!(
            #result;
            wasm_rpc::Responsable::to_response(Ok(wasm_rpc::Value::Null))
        ),
    }
}
