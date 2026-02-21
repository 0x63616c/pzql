use proc_macro2::TokenStream;
use syn::Error;

pub fn expand(_args: TokenStream, input: TokenStream) -> Result<TokenStream, Error> {
    Ok(input)
}
