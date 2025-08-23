use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, LitStr, Token, parse_macro_input, punctuated::Punctuated};

/// Structure representing a single error definition
struct ErrorDef {
    attrs: Vec<syn::Attribute>,
    name: Ident,
    message: LitStr,
}

/// Parser for error list in format: Name => "message"
struct ErrorsList {
    errors: Punctuated<ErrorDef, Token![,]>,
}

impl syn::parse::Parse for ErrorDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let name: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let message: LitStr = input.parse()?;
        Ok(ErrorDef {
            attrs,
            name,
            message,
        })
    }
}

impl syn::parse::Parse for ErrorsList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let errors = input.parse_terminated(ErrorDef::parse, Token![,])?;
        Ok(ErrorsList { errors })
    }
}

/// Procedural macro for declaring multiple errors
///
/// Generates structures for each error and implements the ErrorMarker trait for them.
///
/// # Example
///
/// ```ignore
/// use joinerror::errors;
///
/// errors! {
///     /// Database connection error
///     DatabaseError => "database_error",
///     /// Network communication error  
///     NetworkError => "network_error",
///     ValidationError => "validation_error",
/// }
/// ```
#[proc_macro]
pub fn errors(input: TokenStream) -> TokenStream {
    let errors_list = parse_macro_input!(input as ErrorsList);

    let mut generated = TokenStream2::new();

    for error in &errors_list.errors {
        let attrs = &error.attrs;
        let name = &error.name;
        let message = &error.message;

        let error_impl = quote! {
            #(#attrs)*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct #name;

            impl joinerror::error::ErrorMarker for #name {
                const MESSAGE: &'static str = #message;
            }
        };

        generated.extend(error_impl);
    }

    generated.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_simple_error_parsing() {
        let input: ErrorsList = parse_quote! {
            TestError => "test_error_message",
            AnotherError => "another_error_message"
        };

        assert_eq!(input.errors.len(), 2);
        assert_eq!(input.errors[0].name.to_string(), "TestError");
        assert_eq!(input.errors[0].message.value(), "test_error_message");
    }

    #[test]
    fn test_single_error_parsing() {
        let input: ErrorsList = parse_quote! {
            SingleError => "single_error_message"
        };

        assert_eq!(input.errors.len(), 1);
        assert_eq!(input.errors[0].name.to_string(), "SingleError");
        assert_eq!(input.errors[0].message.value(), "single_error_message");
    }

    #[test]
    fn test_multiple_errors_parsing() {
        let input: ErrorsList = parse_quote! {
            DatabaseError => "database_error",
            NetworkError => "network_error",
            ValidationError => "validation_error",
            AuthenticationError => "authentication_error"
        };

        assert_eq!(input.errors.len(), 4);
        assert_eq!(input.errors[0].name.to_string(), "DatabaseError");
        assert_eq!(input.errors[0].message.value(), "database_error");
        assert_eq!(input.errors[3].name.to_string(), "AuthenticationError");
        assert_eq!(input.errors[3].message.value(), "authentication_error");
    }

    #[test]
    fn test_trailing_comma() {
        let input: ErrorsList = parse_quote! {
            TestError => "test_error_message",
        };

        assert_eq!(input.errors.len(), 1);
        assert_eq!(input.errors[0].name.to_string(), "TestError");
        assert_eq!(input.errors[0].message.value(), "test_error_message");
    }

    #[test]
    fn test_with_documentation() {
        let input: ErrorsList = parse_quote! {
            /// This is a documented error
            DocumentedError => "documented_error",
            /// Another documented error
            /// with multiple lines
            AnotherError => "another_error"
        };

        assert_eq!(input.errors.len(), 2);
        assert_eq!(input.errors[0].name.to_string(), "DocumentedError");
        assert_eq!(input.errors[0].message.value(), "documented_error");
        assert!(!input.errors[0].attrs.is_empty());
        assert_eq!(input.errors[1].name.to_string(), "AnotherError");
        assert_eq!(input.errors[1].message.value(), "another_error");
        assert!(!input.errors[1].attrs.is_empty());
    }
}
