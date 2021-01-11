use darling::{ast::Data, FromDeriveInput, FromVariant};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Generics, Ident};

#[derive(FromDeriveInput)]
#[darling(supports(enum_any))]
pub(crate) struct Receiver {
    ident: Ident,
    generics: Generics,
    data: Data<ReceiverVariant, ()>,
}

impl Receiver {
    fn variants_to_emit(&self) -> Vec<String> {
        self.data
            .as_ref()
            .take_enum()
            .expect("VariantNames only takes enums")
            .into_iter()
            .filter(|v| !v.skip)
            .map(|v| v.ident.to_string())
            .collect()
    }
}

impl ToTokens for Receiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let variants = self.variants_to_emit();
        let variants_len = variants.len();

        tokens.extend(quote! {
            #[automatically_derived]
            impl #impl_generics #ident #ty_generics #where_clause {
                const VARIANTS: [&'static str; #variants_len] = [
                    #(#variants),*
                ];
            }
        })
    }
}

#[derive(FromVariant)]
#[darling(attributes(variant_names))]
struct ReceiverVariant {
    ident: Ident,
    #[darling(default)]
    skip: bool,
}

#[cfg(test)]
mod tests {
    use super::Receiver;
    use darling::FromDeriveInput;
    use syn::parse_quote;

    #[test]
    fn simple() {
        let input = Receiver::from_derive_input(&parse_quote! {
            #[derive(VariantNames)]
            enum Example {
                Hello(String),
                World {
                    planet: String,
                    person: String,
                }
            }
        })
        .unwrap();

        assert_eq!(
            input.variants_to_emit(),
            vec!["Hello".to_string(), "World".to_string()]
        );
    }

    #[test]
    fn skip_variant() {
        let input = Receiver::from_derive_input(&parse_quote! {
            #[derive(VariantNames)]
            enum Example {
                Hello(String),
                #[variant_names(skip)]
                Secret(String),
                World {
                    planet: String,
                    person: String,
                },
            }
        })
        .unwrap();

        assert_eq!(
            input.variants_to_emit(),
            vec!["Hello".to_string(), "World".to_string()]
        );
    }
}
