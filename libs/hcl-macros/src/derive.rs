use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Error, Fields, Result};

use crate::{
    attrs::{validate_label_indices, HclContainerAttrs, HclFieldAttrs},
    codegen::{generate_hcl_impl, HclStruct},
};

/// Expand the HCL derive macro
pub fn expand_derive_hcl(input: &DeriveInput) -> Result<TokenStream> {
    // Parse container-level attributes
    let container_attrs =
        HclContainerAttrs::from_derive_input(input).map_err(|e| Error::new_spanned(input, e))?;

    // Only support structs for now
    let data_struct = match &input.data {
        Data::Struct(data_struct) => data_struct,
        Data::Enum(_) => {
            return Err(Error::new_spanned(
                input,
                "HCL derive macro currently only supports structs",
            ));
        }
        Data::Union(_) => {
            return Err(Error::new_spanned(
                input,
                "HCL derive macro does not support unions",
            ));
        }
    };

    // Only support named fields
    let fields = match &data_struct.fields {
        Fields::Named(fields) => &fields.named,
        Fields::Unnamed(_) => {
            return Err(Error::new_spanned(
                &data_struct.fields,
                "HCL derive macro only supports structs with named fields",
            ));
        }
        Fields::Unit => {
            return Err(Error::new_spanned(
                &data_struct.fields,
                "HCL derive macro does not support unit structs",
            ));
        }
    };

    // Parse field attributes
    let mut parsed_fields = Vec::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_attrs =
            HclFieldAttrs::from_field(field).map_err(|e| Error::new_spanned(field, e))?;

        if !field_attrs.skip {
            parsed_fields.push((field_name, field_attrs, &field.ty));
        }
    }

    // Validate label indices
    let field_refs: Vec<_> = parsed_fields
        .iter()
        .map(|(name, attrs, _)| (*name, attrs))
        .collect();
    validate_label_indices(&field_refs)?;

    // Create HclStruct representation
    let hcl_struct = HclStruct::new(
        &input.ident,
        container_attrs,
        parsed_fields,
        &input.generics,
    );

    // Generate the implementation
    generate_hcl_impl(&hcl_struct)
}
