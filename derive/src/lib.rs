use linked_data_core::attributes::variant::PredicatePath;
use linked_data_core::r#enum::{Enum, Variant};
use linked_data_core::{LinkedDataType, TokenGenerator};
use proc_macro_error::{abort, proc_macro_error};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

#[proc_macro_derive(Sparql, attributes(ld))]
#[proc_macro_error]
pub fn derive_serialize(
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let raw_input = syn::parse_macro_input!(item as DeriveInput);
    let linked_data_type: Result<LinkedDataType<Sparql>, _> =
        LinkedDataType::try_from(raw_input);

    let mut output = TokenStream::new();
    match linked_data_type {
        Ok(linked_data_type) => match linked_data_type {
            LinkedDataType::Enum(e) => e.to_tokens(&mut output),
        },
        Err(err) => panic!("{}", err),
    };

    output.into()
}

struct Sparql;

impl TokenGenerator for Sparql {
    fn generate_type_tokens(
        linked_data_type: &LinkedDataType<Self>,
        tokens: &mut TokenStream,
    ) {
        match linked_data_type {
            LinkedDataType::Enum(enu) => tokens.extend(quote::quote! {#enu}),
        }
    }

    fn generate_enum_tokens(r#enum: &Enum<Self>, tokens: &mut TokenStream) {
        let variants = r#enum.variants();
        let ident = r#enum.ident();

        tokens.extend(quote::quote! {
            impl ToConstructQuery for #ident {
                fn to_query_with_binding(binding_variable: Variable) -> ConstructQuery {
                    ConstructQuery::default()
                     #(#variants)*
                }
            }
        });
    }

    fn generate_variant_tokens(
        variant: &Variant<Self>,
        tokens: &mut TokenStream,
    ) {
        let ty = variant.ty();
        let inner_generator = quote::quote! { #ty::to_query_with_binding };

        let (iri_str, predicate_generator) = match &variant.predicate_path() {
            PredicatePath::Predicate(iri) => (iri.as_str(), inner_generator),
            PredicatePath::ChainedPath {
                to_blank,
                from_blank,
            } => {
                let to_blank_str = to_blank.as_str();
                (
                    from_blank.as_str(),
                    quote::quote! {
                        with_predicate(
                            NamedNode::new_unchecked(#to_blank_str),
                            #inner_generator
                        )
                    },
                )
            }
        };

        tokens.extend(quote::quote! {
            .union_with_binding(
                binding_variable.clone(),
                NamedNode::new_unchecked(#iri_str),
                #predicate_generator
            )
        });
    }
}
