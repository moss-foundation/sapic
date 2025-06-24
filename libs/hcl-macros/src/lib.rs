use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod attrs;
mod codegen;
mod derive;

/// Derive macro for HCL serialization and deserialization
///
/// # Example
/// ```ignore
/// use hcl_macros::Hcl;
///
/// #[derive(Hcl)]
/// #[hcl(
///     block = "struct",
///     rename = "any_struct",
/// )]
/// struct AnyStruct {
///     #[hcl(label(index = 0))]
///     field1: String,
///     #[hcl(attribute(rename = "field2"))]
///     field2: i32,
///     #[hcl(attribute(rename = "field3"))]
///     field3: bool,
/// }
/// ```
#[proc_macro_derive(Hcl, attributes(hcl))]
pub fn derive_hcl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive::expand_derive_hcl(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
