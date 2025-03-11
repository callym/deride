#![feature(proc_macro_diagnostic)]

use std::collections::HashSet;

use proc_macro::{Diagnostic, Level, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Path, parse_macro_input, parse_quote};

#[proc_macro_derive(Without, attributes(without))]
pub fn derive_answer_fn(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let struc = match &input.data {
    Data::Struct(struc) => struc,
    _ => {
      Diagnostic::new(Level::Error, "Without only works on structs").emit();
      return TokenStream::new();
    },
  };

  let original_struc_name = input.ident;

  let without_attr: Attribute = parse_quote!(#[without]);

  let mut name: Option<Path> = None;
  let mut derives: HashSet<Path> = HashSet::new();

  input
    .attrs
    .iter()
    .find(|attr| attr.path() == &parse_quote!(without))
    .unwrap()
    .parse_nested_meta(|meta| {
      let value = meta.value().unwrap().parse::<Path>().unwrap();

      if meta.path.is_ident("name") {
        name = Some(value);
      } else if meta.path.is_ident("derive") {
        derives.insert(value);
      }

      Ok(())
    })
    .unwrap();

  let new_fields = struc
    .fields
    .iter()
    .filter(|field| {
      let without = field.attrs.iter().any(|attr| attr == &without_attr);

      !without
    })
    .collect::<Vec<_>>();

  let new_field_names = new_fields
    .iter()
    .map(|field| field.ident.clone())
    .collect::<Vec<_>>();

  let name = name.unwrap();

  let derives = if derives.is_empty() {
    quote! {}
  } else {
    let derives = derives.iter();
    quote! { #[derive(#(#derives),*)]}
  };

  let expanded = quote! {
    #derives
    struct #name {
      #(#new_fields),*
    }

    impl From<#original_struc_name> for #name {
      fn from(value: #original_struc_name) -> #name {
        let #original_struc_name { #(#new_field_names),*, .. } = value;

        #name {
          #(#new_field_names),*
        }
      }
    }
  };

  TokenStream::from(expanded)
}
