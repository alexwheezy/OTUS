extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(Count)]
pub fn count(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);
    let name = &macro_input.ident;
    let len = match &macro_input.data {
        Data::Enum(enum_item) => enum_item.variants.len(),
        _ => panic!("Works only Enum"),
    };
    quote! {
        impl #name{
            fn count() -> usize{
                #len
            }
        }
    }
    .into()
}
