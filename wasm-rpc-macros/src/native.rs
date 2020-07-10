use proc_macro::TokenStream;
use syn::{parse_quote, Signature, ExprMatch, ItemImpl, ImplItem};

pub fn export_native(tokens: TokenStream) -> TokenStream {
    let mut imp: ItemImpl = syn::parse(tokens.clone()).unwrap();
    let mut matcher: ExprMatch = parse_quote!(match (function, args.as_slice()) {});
    matcher.arms = imp
        .items
        .iter()
        .filter_map(|stmt| {
            if let ImplItem::Method(impl_item) = stmt {
                Some(impl_item)
            } else {
                None
            }
        })
        .filter(|impl_item| impl_item.vis == parse_quote!(pub))
        .map(impl_item_to_arm)
        .collect();
    matcher.arms.push(parse_quote!(
                (f, _) => wasm_rpc::serde_cbor::value::to_value(Err::<(), Box<wasm_rpc::error::Error>>(Box::new(wasm_rpc::error::Error {
               code: 0,
               message: format!("function \"{}\" is undefined", f)
    }))).unwrap()));
    imp.items.push(syn::ImplItem::Method(parse_quote!(
            pub fn call(&mut self, function: &str, args: Vec<serde_cbor::Value>) -> wasm_rpc::serde_cbor::Value {
        #matcher
    }
        )));
    quote!(#imp).into()
}

fn impl_item_to_arm(f: &syn::ImplItemMethod) -> syn::Arm {
    let syn::ImplItemMethod {
        sig: Signature { inputs, ident, .. },
        ..
    } = f;
    let arg_pats = inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(arg) = arg {
                Some(arg)
            } else {
                None
            }
        })
        .cloned()
        .map(|syn::PatType { pat, .. }| *pat)
        .collect::<Vec<syn::Pat>>();
    let args = inputs
        .iter()
        .filter_map(|arg|
                if let syn::FnArg::Typed(arg) = arg {
                    Some(arg)
                } else {
                    None
                }
        )
        .cloned()
        .filter_map(|syn::PatType{pat, ..}| {
                if let syn::Pat::Ident(pat) = *pat {
                    Some(pat)
                } else {
                    None
                }
        })
        .map(|syn::PatIdent{ident, ..}| {
            parse_quote!(wasm_rpc::serde_cbor::value::from_value(#ident.clone()).unwrap())    
        })
        .collect::<Vec<syn::Expr>>();
    let function_name = syn::LitStr::new(&ident.to_string(), proc_macro2::Span::call_site());
    parse_quote!((#function_name, [#(#arg_pats),*]) =>
        wasm_rpc::serde_cbor::value::to_value(self.#ident(#(#args),*)).unwrap()
    )
}
