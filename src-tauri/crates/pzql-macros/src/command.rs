use heck::ToPascalCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{FnArg, Ident, ItemFn, Pat, PatType, Result, Type, parse2};

pub fn expand(_args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let func: ItemFn = parse2(input)?;
    let fn_name = &func.sig.ident;
    let fn_name_str = fn_name.to_string();

    // Collect plain parameters (skip State<T>, AppHandle injected args)
    let plain_params: Vec<(&Ident, &Type)> = func
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                let ty_str = quote!(#ty).to_string();
                if ty_str.contains("State") || ty_str.contains("AppHandle") {
                    return None;
                }
                match pat.as_ref() {
                    Pat::Ident(p) => Some((&p.ident, ty.as_ref())),
                    _ => None,
                }
            }
            FnArg::Receiver(_) => None,
        })
        .collect();

    let struct_name = Ident::new(
        &format!("__{}", fn_name_str.to_pascal_case() + "Args"),
        Span::call_site(),
    );

    let field_names: Vec<&Ident> = plain_params.iter().map(|(n, _)| *n).collect();
    let field_types: Vec<&Type> = plain_params.iter().map(|(_, t)| *t).collect();
    let field_names2 = field_names.clone();

    let is_async = func.sig.asyncness.is_some();
    let call = if is_async {
        quote! { #fn_name(#(a.#field_names2),*).await }
    } else {
        quote! { #fn_name(#(a.#field_names2),*) }
    };

    Ok(quote! {
        #[tauri::command]
        #[specta::specta]
        #func

        #[derive(serde::Deserialize)]
        #[allow(non_camel_case_types)]
        struct #struct_name {
            #(#field_names: #field_types,)*
        }

        inventory::submit! {
            pzql_ipc::WsCommandEntry {
                name: #fn_name_str,
                handler: |args| Box::pin(async move {
                    let a: #struct_name = serde_json::from_value(args)
                        .map_err(|e| serde_json::json!({"message": e.to_string()}))?;
                    let result = #call;
                    serde_json::to_value(result)
                        .map_err(|e| serde_json::json!({"message": e.to_string()}))
                }),
            }
        }
    })
}
