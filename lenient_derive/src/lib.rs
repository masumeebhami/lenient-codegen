use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(LenientDeserialize, attributes(lenient, optional))]
pub fn derive_lenient(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let inner_name = format_ident!("{}Internal", struct_name);

    let Data::Struct(data_struct) = input.data else {
        panic!("#[derive(LenientDeserialize)] only works on structs");
    };

    let Fields::Named(fields_named) = data_struct.fields else {
        panic!("LenientDeserialize only supports named fields");
    };

    let mut inner_fields = vec![];
    let mut from_fields = vec![];

    for field in fields_named.named {
        let ident = field.ident.clone().unwrap();
        let ty = field.ty;
        let mut is_lenient = false;
        let mut is_optional = false;

        for attr in &field.attrs {
            if attr.path().is_ident("lenient") {
                is_lenient = true;
            } else if attr.path().is_ident("optional") {
                is_optional = true;
            }
        }

        let wrapped_ty = if is_optional {
            quote! { ::lenient::Optional<#ty> }
        } else if is_lenient {
            quote! { ::lenient::Lenient<#ty> }
        } else {
            quote! { #ty }
        };

        let serde_default = if is_lenient || is_optional {
            quote! { #[serde(default)] }
        } else {
            quote! {}
        };

        let unwrap_code = if is_optional {
            quote! { #ident: inner.#ident.0.unwrap_or_default() }
        } else if is_lenient {
            quote! { #ident: inner.#ident.0 }
        } else {
            quote! { #ident: inner.#ident }
        };

        inner_fields.push(quote! {
            #serde_default
            #ident: #wrapped_ty
        });

        from_fields.push(unwrap_code);
    }

    let expanded = quote! {
        #[derive(::serde::Deserialize)]
        struct #inner_name {
            #(#inner_fields,)*
        }

        impl<'de> ::serde::Deserialize<'de> for #struct_name {
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
