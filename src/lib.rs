extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;

/// Implementation of a [function-like procedural macro][1].
/// whose function is to... allow the user to print a string
/// to Stdout without the need for ever involving
/// the key typically to the left of the enter key on your keyboard (The quote key).
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn quote_free_printer(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let result = quote! {
        println!(#source);
    };
    result.into()
}

