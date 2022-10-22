extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Validate)]
pub fn validate_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    let id = input.ident;

    let exp = match input.data {
        Data::Enum(de) => {
            let mut variants = vec![];

            for variant in de.variants {
                let var_id = variant.ident;

                match variant.fields {
                    Fields::Named(f) => {
                        let mut fields = vec![];
                        for n in f.named {
                            let f_id = n.ident;
                            fields.push(quote!(#f_id));
                        }

                        variants.push(
                            quote!(if let #id::#var_id { #(#fields, )* } = self { #(#fields.is_valid() &&)* true } else ),
                        );
                    }
                    Fields::Unnamed(_) => {
                        variants.push(quote!(if let #id::#var_id(t) = self { t.is_valid() } else ));
                    }
                    Fields::Unit => {
                        variants.push(quote!(if let #id::#var_id = self { true } else ));
                    }
                }
            }

            variants
        }
        _ => {
            panic!("not supported")
        }
    };

    let expanded = quote!(
        impl symex_lib::Valid for #id {
            fn is_valid(&self) -> bool {
                #(#exp)*
                {
                     symex_lib::ignore_path()
                }
            }
        }
    );
    proc_macro::TokenStream::from(expanded)
}
