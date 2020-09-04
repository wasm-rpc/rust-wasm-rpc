use proc_macro::TokenStream;
use syn::parse::Parser;
use syn::{parse_quote, Block, ExprMatch, Signature};

pub fn export_native(tokens: TokenStream) -> TokenStream {
    if cfg!(test) {
        return tokens;
    }
    let mut stmts: Vec<syn::Stmt> = Block::parse_within.parse(tokens.clone()).unwrap();
    let mut matcher: ExprMatch = parse_quote!(match (function, args.as_slice()) {});
    matcher.arms = stmts
        .iter()
        .filter_map(|stmt| {
            if let syn::Stmt::Item(syn::Item::Fn(f)) = stmt {
                Some(f)
            } else {
                None
            }
        })
        .filter(|f| f.vis == parse_quote!(pub))
        .map(fn_to_arm)
        .collect();
    matcher.arms.push(parse_quote!(
                (f, _) => wasm_rpc::serde_cbor::value::to_value(Err::<(), Box<wasm_rpc::error::Error>>(Box::new(wasm_rpc::error::Error {
               code: 0,
               message: format!("function \"{}\" is undefined", f)
    }))).unwrap()));
    stmts.push(parse_quote!(
            pub fn call<API: ellipticoin::API>(api: &mut API, function: &str, args: Vec<serde_cbor::Value>) -> wasm_rpc::serde_cbor::Value {
        #matcher
        }));
    quote!(
pub mod native {
use super::*;
        #(#stmts)*
    }
        
    ).into()
}

fn fn_to_arm(f: &syn::ItemFn) -> syn::Arm {
    let syn::ItemFn {
        sig: Signature { inputs, ident, .. },
        ..
    } = f;
    let args = inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(syn::PatType { pat, .. }) = arg {
                Some(pat)
            } else {
                None
            }
        })
        .cloned()
        .filter_map(|arg| {
            if let syn::Pat::Ident(syn::PatIdent {
                ident: pat_ident, ..
            }) = *arg
            {
                Some(pat_ident)
            } else {
                None
            }
        })
        .collect::<Vec<syn::Ident>>();
    let (first_arg, rest_args) = args.split_first().expect("expected api parameter");
    let function_name = syn::LitStr::new(&ident.to_string(), proc_macro2::Span::call_site());
    let args2 = rest_args
        .iter()
        .map(|arg| {
            let arg_name = syn::LitStr::new(&arg.to_string(), proc_macro2::Span::call_site());
            parse_quote!(match wasm_rpc::serde_cbor::value::from_value(#arg.clone()){Ok(value) => value, Err(error) => return wasm_rpc::serde_cbor::value::to_value::<std::result::Result<wasm_rpc::serde_cbor::value::Value, String>>(Err(format!("{}: {}",#arg_name, error.to_string()))).unwrap()})
        })
        .collect::<Vec<syn::Expr>>();
    parse_quote!((#function_name, [#(#rest_args),*]) =>
        wasm_rpc::serde_cbor::value::to_value(#ident(#first_arg, #(#args2),*)).unwrap()
    )
}
