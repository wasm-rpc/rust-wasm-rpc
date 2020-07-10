#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

mod native;

use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, FnArg, ItemFn, Result, Signature,
};

#[proc_macro]
pub fn export_native(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    native::export_native(tokens)
}

#[proc_macro]
pub fn export(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Exported::Fns(mut fns) = parse_macro_input!(tokens as Exported);
    if cfg!(debug_assertions) {
        return quote!(#(#fns)*).into();
    };
    for f in fns.iter_mut() {
        replace_values_with_pointers(f);
        return_null(f);
    }
    quote!(#(#fns)*).into()
}

enum Exported {
    Fns(Vec<syn::ItemFn>),
}

impl Parse for Exported {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Exported::Fns(
            syn::Block::parse_within(input)
                .unwrap()
                .iter()
                .cloned()
                .filter_map(|stmt| {
                    if let syn::Stmt::Item(syn::Item::Fn(f)) = stmt {
                        Some(f)
                    } else {
                        None
                    }
                })
                .collect::<Vec<syn::ItemFn>>(),
        ))
    }
}

fn return_null<'a>(f: &'a mut ItemFn) {
    if f.sig.output == parse_quote!() {
        f.sig.output = parse_quote!(-> wasm_rpc::Pointer);
        f.block.stmts.push(syn::Stmt::Expr(parse_quote!(
            wasm_rpc::serde_cbor::Value::Null
        )));
    }
}

fn replace_values_with_pointers(f: &mut ItemFn) {
    let ItemFn {
        sig: Signature { inputs, output, .. },
        block,
        ..
    } = f.clone();
    let pointers: Vec<syn::ExprMethodCall> = f
        .sig
        .inputs
        .clone()
        .into_iter()
        .map(&pointer_to_value)
        .collect();
    f.attrs.push(parse_quote!(#[no_mangle]));
    f.block =
        parse_quote!({wasm_rpc::pointer::from_value(&(|#inputs|#output #block)(#(#pointers),*))});
    f.sig.inputs = f
        .sig
        .inputs
        .iter()
        .cloned()
        .map(|input| {
            if let FnArg::Typed(mut pat) = input {
                pat.ty = parse_quote!(wasm_rpc::Pointer);
                FnArg::Typed(pat)
            } else {
                input
            }
        })
        .collect();
    f.sig.output = parse_quote!(-> wasm_rpc::Pointer);
}

fn pointer_to_value(input: FnArg) -> syn::ExprMethodCall {
    if let FnArg::Typed(syn::PatType { pat, .. }) = input {
        parse_quote!(wasm_rpc::pointer::to_value(#pat).unwrap())
    } else {
        parse_quote!(#input)
    }
}
