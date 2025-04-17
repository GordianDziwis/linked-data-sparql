use linked_data_core::attributes::variant::PredicatePath;
use linked_data_core::r#enum::{Enum, Variant};
use linked_data_core::r#struct::{Field, Struct};
use linked_data_core::{LinkedDataType, TokenGenerator};
use proc_macro_error::proc_macro_error;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

#[proc_macro_error]
#[proc_macro_derive(Sparql, attributes(ld))]
pub fn derive_serialize(
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let raw_input = syn::parse_macro_input!(item as DeriveInput);
    let linked_data_type: Result<LinkedDataType<Sparql>, _> =
        LinkedDataType::try_from(raw_input);

    let mut output = TokenStream::new();
    match linked_data_type {
        Ok(linked_data_type) => linked_data_type.to_tokens(&mut output),
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
        // Remove imports and let the code below use fully qualified paths
        match linked_data_type {
            LinkedDataType::Enum(e) => tokens.extend(quote::quote! {#e}),
            LinkedDataType::Struct(s) => tokens.extend(quote::quote! {#s}),
        }
    }

    fn generate_enum_tokens(r#enum: &Enum<Self>, tokens: &mut TokenStream) {
        let variants = r#enum.variants();
        let ident = r#enum.ident();

        tokens.extend(quote::quote! {
            impl ::linked_data_sparql::ToConstructQuery for #ident {
                fn to_query_with_binding(binding_variable: spargebra::term::Variable) -> ::linked_data_sparql::ConstructQuery {
                    ::linked_data_sparql::ConstructQuery::default()
                     #(#variants)*
                }
            }
        });
    }

    fn generate_struct_tokens(
        r#struct: &Struct<Self>,
        tokens: &mut TokenStream,
    ) {
        let ident = &r#struct.ident;
        let fields = &r#struct.fields;
        let type_tokens = if let Some(type_iri) =
            r#struct.type_iri().map(|iri| iri.clone().into_string())
        {
            quote::quote! {
                .join_with(
                    binding_variable.clone(),
                    // TODO const for type
                    spargebra::term::NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    spargebra::term::NamedNode::new_unchecked(#type_iri),
                )
            }
        } else {
            quote::quote! {}
        };

        tokens.extend(quote::quote! {
            impl ::linked_data_sparql::ToConstructQuery for #ident {
                fn to_query_with_binding(binding_variable: spargebra::term::Variable) -> ::linked_data_sparql::ConstructQuery {
                    ::linked_data_sparql::ConstructQuery::default()
                    #(#fields)*
                    #type_tokens
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
                        ::linked_data_sparql::with_predicate(
                            spargebra::term::NamedNode::new_unchecked(#to_blank_str),
                            #inner_generator
                        )
                    },
                )
            }
        };

        tokens.extend(quote::quote! {
            .union_with_binding(
                binding_variable.clone(),
                spargebra::term::NamedNode::new_unchecked(#iri_str),
                #predicate_generator
            )
        });
    }

    fn generate_field_tokens(field: &Field<Self>, tokens: &mut TokenStream) {
        if field.is_ignored() {
            return;
        }

        if field.is_flattened() {
            let ty = &field.ty;
            tokens.extend(quote::quote! {
                .join(#ty::to_query_with_binding(binding_variable.clone()))
            });
        }

        if let Some(predicate) = field.predicate() {
            let ty = &field.ty;
            let predicate_iri = predicate.as_str();
            tokens.extend(quote::quote! {
                .join_with_binding(
                    binding_variable.clone(),
                    spargebra::term::NamedNode::new_unchecked(#predicate_iri),
                    #ty::to_query_with_binding,
                )
            });
        }
    }
}
