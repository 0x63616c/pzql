use proc_macro2::TokenStream;
use syn::Error;

pub fn expand(input: TokenStream) -> Result<TokenStream, Error> {
    Ok(input)
}
