use linked_data_core::LinkedDataType;
use linked_data_core::r#enum::Enum;
use proc_macro_error::{abort, proc_macro_error};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

mod r#enum;

#[proc_macro_derive(Sparql, attributes(ld))]
#[proc_macro_error]
pub fn derive_serialize(
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // let raw_input = syn::parse_macro_input!(item as DeriveInput);
    // let linked_data_type = LinkedDataType::try_from(raw_input);
    // match linked_data_type {
    //     Ok(linked_data_type) => match linked_data_type {
    //         LinkedDataType::Enum(e) => e.to_tokens(),
    //     },
    //     Err(err) => panic!("{}", err),
    // };
    //
    // let mut output = TokenStream::new();
    //
    // output.into()
    todo!()
}
