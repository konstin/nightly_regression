#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn noop_derive(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // It is required to generate a new token stream; Just returning the
    // input will pass
    let item: syn::Item = syn::parse(input).unwrap();

    quote!(#item).into()
}