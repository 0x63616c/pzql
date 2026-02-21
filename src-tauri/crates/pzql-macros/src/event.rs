use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemStruct, Result};

pub fn expand(input: TokenStream) -> Result<TokenStream> {
    let struct_def: ItemStruct = parse2(input)?;
    let struct_name = &struct_def.ident;
    let event_name = struct_name.to_string().to_snake_case();

    Ok(quote! {
        #struct_def

        impl #struct_name {
            pub fn emit(&self, handle: &impl ::tauri::Emitter<::tauri::Wry>) -> ::tauri::Result<()> {
                handle.emit(#event_name, self)
            }
        }

        inventory::submit! {
            pzql_ipc::WsEventEntry { name: #event_name }
        }
    })
}
