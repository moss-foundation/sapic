use darling::{FromDeriveInput, FromField, FromMeta};
use syn::{Attribute, Error, Ident, Meta, Result};

/// Container-level attributes for HCL derive macro
#[derive(Debug, Clone, Default, FromDeriveInput)]
#[darling(attributes(hcl), default)]
pub struct HclContainerAttrs {
    /// The HCL block type (e.g., "resource", "data", "variable")
    pub block: Option<String>,
    /// Rename the struct in HCL output
    pub rename: Option<String>,
    /// Whether this is a root-level block
    pub root: bool,
}

/// Field-level attributes for HCL derive macro
#[derive(Debug, Clone, Default, FromField)]
#[darling(attributes(hcl), default)]
pub struct HclFieldAttrs {
    /// Mark field as an HCL attribute
    pub attribute: Option<AttributeConfig>,
    /// Mark field as an HCL label
    pub label: Option<LabelConfig>,
    /// Mark field as an HCL block
    pub block: Option<BlockConfig>,
    /// Skip this field during serialization/deserialization
    pub skip: bool,
    /// Flatten this field into the parent
    pub flatten: bool,
    /// Simple attribute flag (for #[hcl(attribute)] without parameters)
    #[darling(default)]
    pub simple_attribute: bool,
}

/// Configuration for HCL attributes
#[derive(Debug, Clone, FromMeta)]
pub struct AttributeConfig {
    /// Rename the attribute in HCL
    pub rename: Option<String>,
    /// Default value for the attribute
    pub default: Option<String>,
    /// Whether this attribute is optional
    #[darling(default)]
    pub optional: bool,
}

impl Default for AttributeConfig {
    fn default() -> Self {
        Self {
            rename: None,
            default: None,
            optional: false,
        }
    }
}

/// Configuration for HCL labels
#[derive(Debug, Clone, Default, FromMeta)]
pub struct LabelConfig {
    /// Index of the label (0-based)
    pub index: Option<usize>,
    /// Name of the label
    pub name: Option<String>,
}

/// Configuration for HCL blocks
#[derive(Debug, Clone, Default, FromMeta)]
pub struct BlockConfig {
    /// Type of the block
    pub block_type: Option<String>,
    /// Whether this is a repeated block
    pub repeated: bool,
}

impl HclFieldAttrs {
    /// Get the field kind (attribute, label, or block)
    pub fn kind(&self) -> FieldKind {
        if self.label.is_some() {
            FieldKind::Label
        } else if self.block.is_some() {
            FieldKind::Block
        } else {
            FieldKind::Attribute
        }
    }

    /// Get the effective name for this field
    pub fn name(&self, field_name: &str) -> String {
        match &self.kind() {
            FieldKind::Attribute => self
                .attribute
                .as_ref()
                .and_then(|attr| attr.rename.clone())
                .unwrap_or_else(|| field_name.to_string()),
            FieldKind::Label => self
                .label
                .as_ref()
                .and_then(|label| label.name.clone())
                .unwrap_or_else(|| field_name.to_string()),
            FieldKind::Block => self
                .block
                .as_ref()
                .and_then(|block| block.block_type.clone())
                .unwrap_or_else(|| field_name.to_string()),
        }
    }

    /// Get the label index for label fields
    pub fn label_index(&self) -> Option<usize> {
        self.label.as_ref().and_then(|label| label.index)
    }

    /// Check if this field is optional
    pub fn is_optional(&self) -> bool {
        match &self.kind() {
            FieldKind::Attribute => self
                .attribute
                .as_ref()
                .map(|attr| attr.optional)
                .unwrap_or(false),
            _ => false,
        }
    }
}

/// The kind of HCL field
#[derive(Debug, Clone, PartialEq)]
pub enum FieldKind {
    /// Regular HCL attribute
    Attribute,
    /// HCL label (for block identification)
    Label,
    /// Nested HCL block
    Block,
}

/// Parse HCL attributes from syn attributes
pub fn parse_hcl_attrs(attrs: &[Attribute]) -> Result<Vec<Meta>> {
    let mut hcl_attrs = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("hcl") {
            match &attr.meta {
                Meta::List(meta_list) => {
                    // Parse nested attributes
                    for _nested in meta_list.tokens.clone() {
                        // This is a simplified parser - in a production version,
                        // you'd want more robust parsing
                        hcl_attrs.push(attr.meta.clone());
                    }
                }
                _ => hcl_attrs.push(attr.meta.clone()),
            }
        }
    }

    Ok(hcl_attrs)
}

/// Validate that label indices are consecutive and start from 0
pub fn validate_label_indices(fields: &[(&Ident, &HclFieldAttrs)]) -> Result<()> {
    let mut label_indices: Vec<(usize, &Ident)> = fields
        .iter()
        .filter_map(|(ident, attrs)| attrs.label_index().map(|idx| (idx, *ident)))
        .collect();

    if label_indices.is_empty() {
        return Ok(());
    }

    label_indices.sort_by_key(|(idx, _)| *idx);

    // Check that indices start from 0 and are consecutive
    for (i, (idx, field_name)) in label_indices.iter().enumerate() {
        if *idx != i {
            return Err(Error::new(
                field_name.span(),
                format!(
                    "Label indices must be consecutive and start from 0. Expected index {}, found {}",
                    i, idx
                ),
            ));
        }
    }

    Ok(())
}
