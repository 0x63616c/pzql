mod command;
mod event;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    command::expand(args.into(), input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn event(_args: TokenStream, input: TokenStream) -> TokenStream {
    event::expand(input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
