use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, Ident, Result, Type};

use crate::attrs::{FieldKind, HclContainerAttrs, HclFieldAttrs};

/// Represents a parsed HCL struct
pub struct HclStruct<'a> {
    pub name: &'a Ident,
    pub container_attrs: HclContainerAttrs,
    pub fields: Vec<HclField<'a>>,
    pub generics: &'a Generics,
}

/// Represents a field in an HCL struct
pub struct HclField<'a> {
    pub name: &'a Ident,
    pub attrs: HclFieldAttrs,
    pub ty: &'a Type,
}

impl<'a> HclStruct<'a> {
    pub fn new(
        name: &'a Ident,
        container_attrs: HclContainerAttrs,
        fields: Vec<(&'a Ident, HclFieldAttrs, &'a Type)>,
        generics: &'a Generics,
    ) -> Self {
        let fields = fields
            .into_iter()
            .map(|(name, attrs, ty)| HclField { name, attrs, ty })
            .collect();

        Self {
            name,
            container_attrs,
            fields,
            generics,
        }
    }

    /// Get the effective block type name
    pub fn block_type(&self) -> String {
        self.container_attrs
            .block
            .clone()
            .unwrap_or_else(|| "block".to_string())
    }

    /// Get the effective struct name for HCL
    pub fn hcl_name(&self) -> String {
        self.container_attrs
            .rename
            .clone()
            .unwrap_or_else(|| self.name.to_string().to_lowercase())
    }

    /// Get label fields sorted by index
    pub fn label_fields(&self) -> Vec<&HclField<'a>> {
        let mut labels: Vec<_> = self
            .fields
            .iter()
            .filter(|field| field.attrs.kind() == FieldKind::Label)
            .collect();

        labels.sort_by_key(|field| field.attrs.label_index().unwrap_or(0));
        labels
    }

    /// Get attribute fields
    pub fn attribute_fields(&self) -> Vec<&HclField<'a>> {
        self.fields
            .iter()
            .filter(|field| field.attrs.kind() == FieldKind::Attribute)
            .collect()
    }

    /// Get block fields
    pub fn block_fields(&self) -> Vec<&HclField<'a>> {
        self.fields
            .iter()
            .filter(|field| field.attrs.kind() == FieldKind::Block)
            .collect()
    }
}

/// Generate the HCL implementation
pub fn generate_hcl_impl(hcl_struct: &HclStruct) -> Result<TokenStream> {
    let struct_name = hcl_struct.name;
    let (impl_generics, ty_generics, where_clause) = hcl_struct.generics.split_for_impl();

    let serialize_impl = generate_serialize_impl(hcl_struct)?;
    let deserialize_impl = generate_deserialize_impl(hcl_struct)?;
    let helper_methods = generate_helper_methods();
    let block_type = &hcl_struct.block_type();
    let hcl_name = &hcl_struct.hcl_name();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Serialize to HCL format
            pub fn to_hcl(&self) -> ::std::result::Result<String, Box<dyn ::std::error::Error>> {
                #serialize_impl
            }

            /// Deserialize from HCL format
            pub fn from_hcl(input: &str) -> ::std::result::Result<Self, Box<dyn ::std::error::Error>> {
                #deserialize_impl
            }

            /// Get the HCL block type for this struct
            pub fn hcl_block_type() -> &'static str {
                #block_type
            }

            /// Get the HCL name for this struct
            pub fn hcl_name() -> &'static str {
                #hcl_name
            }

            #helper_methods
        }
    })
}

/// Generate serialization implementation
fn generate_serialize_impl(hcl_struct: &HclStruct) -> Result<TokenStream> {
    let block_type = &hcl_struct.block_type();
    let _hcl_name = &hcl_struct.hcl_name();

    let label_fields = hcl_struct.label_fields();
    let attribute_fields = hcl_struct.attribute_fields();
    let block_fields = hcl_struct.block_fields();

    // Generate label serialization
    let label_serialization = if !label_fields.is_empty() {
        let label_parts: Vec<_> = label_fields
            .iter()
            .map(|field| {
                let field_name = field.name;
                quote! { format!("\"{}\"", self.#field_name) }
            })
            .collect();

        quote! {
            let labels = vec![#(#label_parts),*].join(" ");
        }
    } else {
        quote! { let labels = String::new(); }
    };

    // Generate attribute serialization
    let attribute_serialization = if !attribute_fields.is_empty() {
        let attr_parts: Vec<_> = attribute_fields
            .iter()
            .map(|field| {
                let field_name = field.name;
                let hcl_name = field.attrs.name(&field_name.to_string());

                if field.attrs.is_optional() {
                    quote! {
                        if let Some(ref value) = self.#field_name {
                            attributes.push(format!("  {} = {}", #hcl_name, Self::serialize_value(value)?));
                        }
                    }
                } else {
                    quote! {
                        attributes.push(format!("  {} = {}", #hcl_name, Self::serialize_value(&self.#field_name)?));
                    }
                }
            })
            .collect();

        quote! {
            let mut attributes = Vec::new();
            #(#attr_parts)*
            let attributes_str = attributes.join("\n");
        }
    } else {
        quote! { let attributes_str = String::new(); }
    };

    // Generate block serialization
    let block_serialization = if !block_fields.is_empty() {
        let block_parts: Vec<_> = block_fields
            .iter()
            .map(|field| {
                let field_name = field.name;
                quote! {
                    blocks.push(self.#field_name.to_hcl()?);
                }
            })
            .collect();

        quote! {
            let mut blocks = Vec::new();
            #(#block_parts)*
            let blocks_str = blocks.join("\n");
        }
    } else {
        quote! { let blocks_str = String::new(); }
    };

    Ok(quote! {
        #label_serialization
        #attribute_serialization
        #block_serialization

        let mut result = String::new();

        // Write block header
        if !labels.is_empty() {
            result.push_str(&format!("{} {} {{\n", #block_type, labels));
        } else {
            result.push_str(&format!("{} {{\n", #block_type));
        }

        // Write attributes
        if !attributes_str.is_empty() {
            result.push_str(&attributes_str);
            result.push('\n');
        }

        // Write nested blocks
        if !blocks_str.is_empty() {
            result.push_str(&blocks_str);
            result.push('\n');
        }

        result.push_str("}\n");

        Ok(result)
    })
}

/// Generate deserialization implementation
fn generate_deserialize_impl(hcl_struct: &HclStruct) -> Result<TokenStream> {
    let struct_name = hcl_struct.name;
    let label_fields = hcl_struct.label_fields();
    let attribute_fields = hcl_struct.attribute_fields();
    let block_fields = hcl_struct.block_fields();

    // Generate field initializations
    let mut field_inits = Vec::new();

    // Initialize label fields
    for (i, field) in label_fields.iter().enumerate() {
        let field_name = field.name;
        field_inits.push(quote! {
            #field_name: labels.get(#i)
                .ok_or_else(|| format!("Missing label at index {}", #i))?
                .clone()
        });
    }

    // Initialize attribute fields
    for field in &attribute_fields {
        let field_name = field.name;
        let hcl_name = field.attrs.name(&field_name.to_string());

        if field.attrs.is_optional() {
            field_inits.push(quote! {
                #field_name: attributes.get(#hcl_name).cloned()
            });
        } else {
            field_inits.push(quote! {
                #field_name: attributes.get(#hcl_name)
                    .ok_or_else(|| format!("Missing required attribute: {}", #hcl_name))?
                    .clone()
            });
        }
    }

    // Initialize block fields
    for field in &block_fields {
        let field_name = field.name;
        let field_type = field.ty;
        field_inits.push(quote! {
            #field_name: <#field_type>::from_hcl(&blocks_content)?
        });
    }

    Ok(quote! {
        // This is a simplified parser - in a production version,
        // you'd want to use a proper HCL parser library
        let (labels, attributes, blocks_content) = Self::parse_hcl_content(input)?;

        Ok(#struct_name {
            #(#field_inits),*
        })
    })
}

/// Generate helper methods for serialization/deserialization
pub fn generate_helper_methods() -> TokenStream {
    quote! {
        /// Serialize a value to HCL format
        fn serialize_value<T: ::std::fmt::Display>(value: &T) -> ::std::result::Result<String, Box<dyn ::std::error::Error>> {
            // This is a simplified implementation
            // In a production version, you'd handle different types properly
            Ok(format!("\"{}\"", value))
        }

        /// Parse HCL content into components
        fn parse_hcl_content(input: &str) -> ::std::result::Result<(Vec<String>, std::collections::HashMap<String, String>, String), Box<dyn ::std::error::Error>> {
            // This is a placeholder implementation
            // In a production version, you'd use a proper HCL parser
            Ok((Vec::new(), std::collections::HashMap::new(), String::new()))
        }
    }
}
