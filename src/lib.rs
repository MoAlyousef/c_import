#![doc = include_str!("../README.md")]

#![allow(clippy::needless_doctest_main)]

use proc_macro::TokenStream;

mod utils;

#[proc_macro]
pub fn c_import(input: TokenStream) -> TokenStream {
    utils::common(input, false)
}

#[proc_macro]
pub fn cpp_import(input: TokenStream) -> TokenStream {
    utils::common(input, true)
}
