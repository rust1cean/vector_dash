//! This crate provides a procedural macro for implementing setters for struct builder fields.
//!
//! # Example:
//!
//! In the example, the procedural macro [`With`](With) is added
//! to the attributes of the `Builder` structure, which implements the
//! `Builder::with_number` and `Builder::with_string` methods.
//!
//! ```rust
//! use builder_with::With;
//!
//! #[derive(With, Default, Debug)]
//! struct Builder {
//!     number: f64,
//!     string: String,
//! }
//!
//! assert_eq!(
//!     Builder { number: 50.0, string: String::from("hello world!") },
//!     Builder::default()
//!         .with_number(50.)
//!         .with_string("hello world!".to_owned())
//! );
//! ```
//!
//! The code above translates into the listing below:
//!
//! ```rust
//! use builder_with::With;
//!
//! #[derive(With, Default, Debug)]
//! struct Builder {
//!     number: f64,
//!     string: String,
//! }
//!
//! impl Builder {
//!     pub fn with_number(&mut self, number: f64) -> &mut Self {
//!         self.number = number;
//!         self
//!     }
//!
//!     pub fn with_string(&mut self, string: String) -> &mut Self {
//!         self.string = string;
//!         self
//!     }
//! }
//!
//! assert_eq!(
//!     Builder { number: 50.0, string: String::from("hello world!") },
//!     Builder::default()
//!         .with_number(50.)
//!         .with_string("hello world!".to_owned())
//! );
//! ```

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[proc_macro_derive(With)]
pub fn setters(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let named_fields = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => named,
            _ => panic!("Expected named fields"),
        },
        _ => panic!("Expected struct"),
    };

    let field = named_fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect::<Vec<_>>();
    let field_type = named_fields
        .iter()
        .map(|field| &field.ty)
        .collect::<Vec<_>>();

    let mut methods = Vec::new();

    for (field_type, field_name) in field_type.iter().zip(field.iter()) {
        let method_name: String = format!("with_{}", field_name);
        let method_name: proc_macro2::TokenStream = method_name.parse().unwrap();

        methods.push(quote! {
            pub fn #method_name(&mut self, #field_name: #field_type) -> &mut Self {
                self.#field_name = #field_name;
                self
            }
        });
    }

    quote! {
        impl #ident {
            #(#methods)*
        }
    }
    .into()
}
