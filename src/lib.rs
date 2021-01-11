extern crate proc_macro;

use darling::FromDeriveInput;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod fields;
mod variants;

#[proc_macro_derive(FieldNames, attributes(field_names))]
pub fn derive_field_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    fields::Receiver::from_derive_input(&parse_macro_input!(input as DeriveInput))
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}

#[proc_macro_derive(VariantNames, attributes(variant_names))]
pub fn derive_variant_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    variants::Receiver::from_derive_input(&parse_macro_input!(input as DeriveInput))
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}
