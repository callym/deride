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

  let without_attr: Attribute = parse_quote!(#[without]);

  let withouts = input
    .attrs
    .iter()
    .filter(|attr| attr.path() == &parse_quote!(without))
    .map(|attr| {
      let mut name: Option<Path> = None;
      let mut derives: HashSet<Path> = HashSet::new();

      attr.parse_nested_meta(|meta| {
        let value = meta.value().unwrap().parse::<Path>().unwrap();

        if meta.path.is_ident("name") {
          name = Some(value);
        } else if meta.path.is_ident("derive") {
          derives.insert(value);
        }

        Ok(())
      }).unwrap();

      let name = name.unwrap();

      Without { name, derives }
    })
    .collect::<Vec<_>>();

  let mut expanded = Vec::new();

  #[allow(clippy::never_loop)]
  for Without { name, derives } in withouts {
    let new_fields = struc
      .fields
      .iter()
      .filter(|field| {
        let without = field
          .attrs
          .iter()
          .any(|attr| attribute(&without_attr, &name, attr));

        !without
      })
      .map(|field| {
        let mut field = field.clone();

        field.attrs.retain(|attr| attr.path() != without_attr.path());

        field
      })
      .collect::<Vec<_>>();

    let new_field_names = new_fields
      .iter()
      .map(|field| field.ident.clone())
      .collect::<Vec<_>>();

    let derives = if derives.is_empty() {
      quote! {}
    } else {
      let derives = derives.iter();
      quote! { #[derive(#(#derives),*)]}
    };

    let vis = input.vis.clone();
    let original_name = input.ident.clone();

    expanded.push(quote! {
      #derives
      #vis struct #name {
        #(#new_fields),*
      }

      impl From<#original_name> for #name {
        fn from(value: #original_name) -> #name {
          let #original_name { #(#new_field_names),*, .. } = value;

          #name {
            #(#new_field_names),*
          }
        }
      }
    });
  }

  let expanded = quote! {
    #(#expanded)*
  };

  TokenStream::from(expanded)
}

#[derive(Debug)]
struct Without {
  name: Path,
  derives: HashSet<Path>,
}

fn attribute(without_attr: &Attribute, name: &Path, attribute: &Attribute) -> bool {
  if attribute.path().get_ident() != without_attr.path().get_ident() {
    return false;
  }

  let mut matching_path = false;

  attribute
    .parse_nested_meta(|meta| {
      matching_path = &meta.path == name;

      Ok(())
    })
    .unwrap();

  matching_path
}
