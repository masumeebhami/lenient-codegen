use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(LenientDeserialize)]
pub fn derive_lenient(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let inner_name = format_ident!("{}Internal", name);

    let Data::Struct(data_struct) = &input.data else {
        panic!("#[derive(LenientDeserialize)] only works on structs");
    };

    let fields: Vec<_> = match &data_struct.fields {
        Fields::Named(fields_named) => fields_named.named.iter().collect(),
        _ => panic!("Only named fields are supported"),
    };

    let inner_fields = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            #[serde(default)]
            #ident: ::lenient::Lenient<#ty>
        }
    });

    let from_fields = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        quote! { #ident: inner.#ident.0 }
    });

    let expanded = quote! {
        #[derive(::serde::Deserialize)]
        struct #inner_name {
            #(#inner_fields,)*
        }

        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D>(de: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let inner = #inner_name::deserialize(de)?;
                Ok(Self {
                    #(#from_fields,)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}