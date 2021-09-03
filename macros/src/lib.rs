extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// Creates a `derive` macro for a marker type, with no methods implemented.
macro_rules! derive_marker {
    ($($type:ident),*) => {
        $(
            #[proc_macro_derive($type)]
            #[allow(non_snake_case)]
            pub fn $type(input: TokenStream) -> TokenStream {
                // Construct a representation of Rust code as a syntax tree
                // that we can manipulate
                let ast: syn::DeriveInput = syn::parse(input).unwrap();

                // Build the trait implementation
                let name = &ast.ident;
                (quote! {
                    impl $type for #name {}
                })
                .into()
            }
        )*
    };
}

derive_marker!(OpMarker, UnOpMarker, BinOpMarker);
