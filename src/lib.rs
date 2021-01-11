extern crate proc_macro;

use darling::FromDeriveInput;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod fields;

#[proc_macro_derive(FieldNames, attributes(field_names))]
pub fn derive_field_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    fields::Receiver::from_derive_input(&parse_macro_input!(input as DeriveInput))
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}
