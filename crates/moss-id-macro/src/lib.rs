use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, Token, bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
};

/// Top-level parser expects a bracketed, comma-separated list: `[ ... ]`
struct IdList(Vec<Ident>);

impl Parse for IdList {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        bracketed!(content in input);
        let items: Punctuated<Ident, Token![,]> =
            content.parse_terminated(Ident::parse, Token![,])?;
        Ok(IdList(items.into_iter().collect()))
    }
}

#[proc_macro]
pub fn ids(input: TokenStream) -> TokenStream {
    let IdList(items) = syn::parse_macro_input!(input as IdList);

    // default fixed nanoid length
    let default_len = 10usize;

    let mut all_tokens = proc_macro2::TokenStream::new();

    for name in items {
        let expanded = quote! {
            /// @category Primitive
            #[derive(Clone, Debug, PartialEq, Hash, Eq, ::serde::Serialize, ::serde::Deserialize)]
            #[serde(transparent)]
            pub struct #name(::std::sync::Arc<::std::string::String>);

            impl #name {
                pub fn new() -> Self {
                    Self(::std::sync::Arc::new(::nanoid::nanoid!(#default_len)))
                }

                pub fn inner(&self) -> ::std::sync::Arc<::std::string::String> {
                    self.0.clone()
                }
            }

            impl<T> ::std::convert::From<T> for #name
            where T: ::std::convert::Into<::std::sync::Arc<::std::string::String>> {
                fn from(s: T) -> Self {
                    Self(s.into())
                }
            }

            impl ::std::convert::AsRef<str> for #name {
                fn as_ref(&self) -> &str {
                    self.0.as_ref()
                }
            }

            impl ::std::fmt::Display for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }

            impl ::std::ops::Deref for #name {
                type Target = ::std::sync::Arc<String>;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            #[rustfmt::skip]
            impl ::ts_rs::TS for #name {
                type WithoutGenerics = Self;
                type OptionInnerType = Self;

                fn name() -> ::std::string::String { "string".to_string() }
                fn inline() -> ::std::string::String { "string".to_string() }
                fn inline_flattened() -> ::std::string::String { "string".to_string() }
                fn decl() -> ::std::string::String { unreachable!() }
                fn decl_concrete() -> ::std::string::String { unreachable!() }
                fn dependencies() -> ::std::vec::Vec<::ts_rs::Dependency> { ::std::vec::Vec::new() }
            }
        };

        all_tokens.extend(expanded);
    }

    all_tokens.into()
}
