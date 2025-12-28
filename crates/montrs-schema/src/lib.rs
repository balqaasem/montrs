extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, LitInt};

#[proc_macro_derive(Schema, attributes(schema))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut all_field_validations = Vec::new();

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            for f in fields.named {
                let field_name = f.ident.clone();
                for attr in f.attrs {
                    if attr.path().is_ident("schema") {
                        let _ = attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("min_len") {
                                let value = meta.value()?;
                                let lit: LitInt = value.parse()?;
                                let min = lit.base10_parse::<usize>()?;
                                all_field_validations.push(quote! {
                                    if self.#field_name.len() < #min {
                                        return Err(format!("{} is too short (min {})", stringify!(#field_name), #min));
                                    }
                                });
                            } else if meta.path.is_ident("email") {
                                all_field_validations.push(quote! {
                                    if !self.#field_name.contains('@') {
                                        return Err(format!("{} must be a valid email", stringify!(#field_name)));
                                    }
                                });
                            }
                            Ok(())
                        });
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl #name {
            pub fn validate(&self) -> Result<(), String> {
                #(#all_field_validations)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
