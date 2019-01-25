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

use proc_macro2::TokenStream;

#[proc_macro_attribute]
pub fn export(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let f: syn::ItemFn =
        syn::parse(input.clone()).expect("`export` can only be applied to a function");

    let syn::ItemFn {
        ident,
        decl: box decl,
        vis,
        block,
        ..
    } = f.clone();
    let syn::FnDecl {
        inputs,
        output,
        fn_token,
        ..
    } = decl;
    let new_inputs = rewrite_inputs_as_pointers(inputs.clone());
    let result = inputs_to_pointers(output.clone(), inputs, block);
    let response = wrap_result(output, result);

    quote!(
        #[cfg(not(test))]
        #[no_mangle]
        #vis #fn_token #ident (#(#new_inputs),*) -> wasm_rpc::Pointer
        {
            #[cfg(debug_assertions)]
            wasm_rpc::hook();
            #response
        }
        #[cfg(test)]
        #f
    )
    .into()
}

#[proc_macro]
pub fn require(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut inputs: Vec<TokenStream> = macro_arguments(input.into());
    let error = inputs.pop();
    let condition = inputs.pop();

    (quote!(if(!(#condition)){return Err(#error);})).into()
}
#[proc_macro]
pub fn get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inputs: Vec<TokenStream> = macro_arguments(input.into());
    let key = concat_vecs(inputs);

    (quote!(wasm_rpc::get_memory(#key))).into()
}

#[proc_macro]
pub fn set(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut inputs: Vec<TokenStream> = macro_arguments(input.into());
    let value = inputs.pop();

    let key = concat_vecs(inputs);

    quote!(wasm_rpc::set_memory(#key, #value)).into()
}

#[proc_macro]
pub fn load(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inputs: Vec<TokenStream> = macro_arguments(input.into());
    let key = concat_vecs(inputs);

    (quote!(wasm_rpc::get_storage(#key))).into()
}

#[proc_macro]
pub fn store(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut inputs: Vec<TokenStream> = macro_arguments(input.into());
    let value = inputs.pop();
    let key = concat_vecs(inputs);

    quote!(wasm_rpc::set_storage(#key, #value)).into()
}

fn rewrite_inputs_as_pointers(
    inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> Vec<TokenStream> {
    inputs
        .clone()
        .into_iter()
        .map(|input| match input.clone() {
            syn::FnArg::Captured(syn::ArgCaptured { pat, .. }) => quote!(#pat: wasm_rpc::Pointer),
            input => quote!(#input),
        })
        .collect()
}
fn inputs_to_pointers(
    return_type: syn::ReturnType,
    inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    block: Box<syn::Block>,
) -> TokenStream {
    let pointers = inputs
        .clone()
        .into_iter()
        .map(|input| match input.clone() {
            syn::FnArg::Captured(syn::ArgCaptured {
                pat: syn::Pat::Ident(syn::PatIdent { ident, .. }),
                ty: syn::Type::Path(syn::TypePath { path, .. }),
                ..
            }) => {
                let dref_fn = match quote!(#path).to_string().as_ref() {
                    "BTreeMap < ObjectKey , Value >" => {
                        quote!(wasm_rpc::Dereferenceable::to_object)
                    }
                    "String" => quote!(wasm_rpc::Dereferenceable::to_string),
                    "Vec < u8 >" => quote!(wasm_rpc::Dereferenceable::to_bytes),
                    "i64" => quote!(wasm_rpc::Dereferenceable::to_i64),
                    "i32" => quote!(wasm_rpc::Dereferenceable::to_i64),
                    "i16" => quote!(wasm_rpc::Dereferenceable::to_i64),
                    "i8" => quote!(wasm_rpc::Dereferenceable::to_i64),
                    "u64" => quote!(wasm_rpc::Dereferenceable::to_i64),
                    "u32" => quote!(wasm_rpc::Dereferenceable::to_i64),
                    "u16" => quote!(wasm_rpc::Dereferenceable::to_i64),
                    "u8" => quote!(wasm_rpc::Dereferenceable::to_i64),
                    _ => quote!(),
                };

                let is_integer = match quote!(#path).to_string().as_ref() {
                    "i64" => true,
                    "i32" => true,
                    "i16" => true,
                    "i8" => true,
                    "u64" => true,
                    "u32" => true,
                    "u16" => true,
                    "u8" => true,
                    _ => false,
                };

                if is_integer {
                    quote!(#dref_fn(&#ident) as #path)
                } else {
                    quote!(#dref_fn(&#ident))
                }
            }
            _ => quote!(),
        })
        .collect::<Vec<TokenStream>>();
    quote!((|#(#inputs),*|#return_type{#block})(#(#pointers),*))
}
fn wrap_result(return_type: syn::ReturnType, result: TokenStream) -> TokenStream {
    match return_type.clone() {
        syn::ReturnType::Default => quote!(
        #result;
        wasm_rpc::Responsable::to_response(Ok(wasm_rpc::Value::Null))
        ),
        syn::ReturnType::Type(syn::token::RArrow(_), box path) => {
            if quote!(#path).to_string().starts_with("Result") {
                quote!(
                wasm_rpc::Responsable::to_response((#result))
                )
            } else {
                quote!(
                wasm_rpc::Responsable::to_response(Ok(#result))
                )
            }
        }
    }
}
fn macro_arguments(tokens: TokenStream) -> Vec<TokenStream> {
    tokens
        .into_iter()
        .map(|token| {
            let tt: proc_macro2::TokenTree = token.into();
            quote!(#tt)
        })
        .collect::<Vec<TokenStream>>()
        .split(|token| match syn::parse(token.clone().into()) {
            Ok(syn::token::Comma { .. }) => true,
            _ => false,
        })
        .map(|items| quote!(#(#items)*))
        .collect()
}

fn concat_vecs(vecs: Vec<TokenStream>) -> TokenStream {
    let slices: Vec<TokenStream> = vecs
        .iter()
        .map(|argument| {
            let slice = match syn::parse(argument.clone().into()) {
                Ok(syn::LitStr { .. }) => quote!(#argument.as_bytes()),
                _ => quote!(#argument.as_slice()),
            };

            quote!(#slice)
        })
        .collect();
    quote! {
        [#(#slices),*].concat()
    }
}
