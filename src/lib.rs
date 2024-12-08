#![doc = include_str!("../README.md")]
#![no_std]

extern crate proc_macro;

use quote::quote;
use syn::Data;

#[proc_macro_derive(IntoEnum, attributes(into_enum))]
pub fn derive_from_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput {
        ident,
        data,
        generics,
        ..
    } = syn::parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Data::Enum(data) = data else {
        return syn::Error::new(ident.span(), "Only enums can derive IntoEnum")
            .to_compile_error()
            .into();
    };

    let mut stream = proc_macro2::TokenStream::new();
    'variants: for variant in data.variants {
        for attr in &variant.attrs {
            if attr.path().is_ident("into_enum") {
                let mut skip = false;
                if let Err(err) = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        skip = true;
                    }
                    Ok(())
                }) {
                    return err.to_compile_error().into();
                };
                if skip {
                    continue 'variants;
                }
            }
        }
        let fields = match variant.fields {
            syn::Fields::Unit => syn::punctuated::Punctuated::new(),
            syn::Fields::Unnamed(fields) => fields.unnamed,
            syn::Fields::Named(_) => continue,
        };
        let variant_ident = variant.ident;
        match fields.len() {
            0 => stream.extend(quote! {
                impl #impl_generics ::std::convert::From<()> for #ident #ty_generics #where_clause {
                    fn from(value: ()) -> Self {
                      Self::#variant_ident
                    }
                }
            }),
            1 => {
                let ty = fields.into_iter().next().unwrap().ty;
                stream.extend(quote! {
                    impl #impl_generics ::std::convert::From<#ty> for #ident #ty_generics #where_clause {
                        fn from(value: #ty) -> Self {
                          Self::#variant_ident(value)
                        }
                    }
                })
            }
            field_count => {
                let ty = syn::Type::Tuple(syn::TypeTuple {
                    paren_token: syn::token::Paren::default(),
                    elems: syn::punctuated::Punctuated::from_iter(fields.into_iter().map(|f| f.ty)),
                });
                let idx = (0..field_count).map(syn::Index::from);
                stream.extend(quote! {
                    impl #impl_generics ::std::convert::From<#ty> for #ident {
                        fn from(value: #ty) -> Self {
                            Self::#variant_ident(#(value.#idx),*)
                        }
                    }
                })
            }
        };
    }
    stream.into()
}
