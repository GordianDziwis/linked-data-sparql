

// impl ToTokens for SparqlVariant<'_> {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let ty = self.model.r#type;
//         let inner_generator = quote::quote! { #ty::to_query_with_binding };
//
//         let (iri_str, predicate_generator) = match &self.model.attributes {
//             SparqlIriAttributes::One(iri) => (iri.as_str(), inner_generator),
//             SparqlIriAttributes::Both {
//                 inner_iri,
//                 outer_iri,
//             } => {
//                 let inner_iri_str = inner_iri.as_str();
//                 (
//                     outer_iri.as_str(),
//                     quote::quote! {
//                         with_predicate(
//                             NamedNode::new_unchecked(#inner_iri_str),
//                             #inner_generator
//                         )
//                     },
//                 )
//             }
//         };
//
//         tokens.extend(quote::quote! {
//             .union_with_binding(
//                 binding_variable.clone(),
//                 NamedNode::new_unchecked(#iri_str),
//                 #predicate_generator
//             )
//         });
//     }
// }

// pub fn generate(derive_input: DeriveInput) -> TokenStream {
//     let x: LinkedDataType<Sparql> = TryFrom::try_from(derive_input).unwrap();
//     // match x {
//     //     LinkedDataType::Enum(x) => x.to_tokens(),
//     // };
//
//     let tokens = quote::quote!(#x);
//     todo!()
//     // x.to_tokens
// }
