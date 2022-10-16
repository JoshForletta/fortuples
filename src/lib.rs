//! 🚧📝 Documentation coming soon 📝🚧

use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Error};

mod expand;
mod parse;
mod types;

use types::{AutoImplInfo, FortuplesInfo};

#[proc_macro]
/// 🚧📝 Documentation coming soon 📝🚧
pub fn fortuples(item: TokenStream) -> TokenStream {
    let info = parse_macro_input!(item as FortuplesInfo);

    match info.expand() {
        Ok(tokens) => tokens,
        Err(e) => e.into_compile_error(),
    }
    .into()
}

#[proc_macro_attribute]
/// 🚧📝 Documentation coming soon 📝🚧
pub fn auto_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return Error::new(
            proc_macro2::TokenStream::from(attr).span(),
            "`auto_impl` doesn't take arguments",
        )
        .into_compile_error()
        .into();
    }

    let info = parse_macro_input!(item as AutoImplInfo);

    match info.expand() {
        Ok(tokens) => tokens,
        Err(e) => e.into_compile_error(),
    }
    .into()
}
