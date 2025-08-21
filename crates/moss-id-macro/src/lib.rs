use quote::quote;
use syn::{Ident, parse_macro_input};

#[proc_macro]
pub fn generate_id_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let name = parse_macro_input!(input as Ident);

    let expanded = quote! {
        /// @category Primitive
        #[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct #name(::std::sync::Arc<::std::string::String>);

        impl #name {
            pub fn new() -> Self {
                // call the nanoid macro. We assume the `nanoid!` macro is available
                // in scope as `::nanoid::nanoid!` (the crate's macro); adjust if you
                // import it differently in your project.
                Self(::std::sync::Arc::new(::nanoid::nanoid!(10)))
            }
        }

        impl ::std::convert::From<::std::string::String> for #name {
            fn from(s: ::std::string::String) -> Self {
                Self(::std::sync::Arc::new(s))
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

    expanded.into()
}
