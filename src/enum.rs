use std::collections::HashMap;

use iref::IriBuf;
use linked_data_core::{TypeAttributes, VariantAttributes};
use proc_macro_error::abort;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::{Visit, visit_data_enum};
use syn::{Attribute, DataEnum, Field, Type, Variant};

impl ToTokens for SparqlEnum<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variants = &self.variants;
        let ident = self.attributes.ident;

        tokens.extend(quote::quote! {
			impl ToConstructQuery for #ident {
				fn to_query_with_binding(binding_variable: Variable) -> ConstructQuery {
					ConstructQuery::default()
					 #(#variants)*
				}
			}
		});
    }
}

impl ToTokens for SparqlVariant<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = self.ty;
        let inner_generator = quote::quote! { #ty::to_query_with_binding };
        println!("{:?}", &self.iri_attributes);

        let (iri_str, predicate_generator) = match &self.iri_attributes {
            SparqlIriAttributes::One(iri) => (iri.as_str(), inner_generator),
            SparqlIriAttributes::Both {
                inner_iri,
                outer_iri,
            } => {
                let inner_iri_str = inner_iri.as_str();
                (
                    outer_iri.as_str(),
                    quote::quote! {
                        with_predicate(
                            NamedNode::new_unchecked(#inner_iri_str),
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

impl<'ast> Visit<'ast> for SparqlEnum<'ast> {
    fn visit_variant(&mut self, i: &'ast Variant) {
        // let sparql_variant = SparqlVariant::from_variant(i, self.prefixes);
        // self.variants.push(sparql_variant);
    }
}
pub fn generate(
    attrs: TypeAttributes,
    ident: Ident,
    e: syn::DataEnum,
) -> TokenStream {
    let mut visitor = SparqlEnum {
        variants: vec![],
        attributes: attrs,
    };
    visitor.visit_data_enum(&e);

    let tokens = quote::quote!(#visitor);
    tokens
}
