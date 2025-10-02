use crate::models::primitives::{
    EntryClass, EntryId, EntryProtocol, FormDataParamId, HeaderId, PathParamId, QueryParamId,
};
use hcl::Expression;
use indexmap::IndexMap;
use moss_hcl::{Block, LabeledBlock, deserialize_expression, expression, serialize_expression};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadataSpec {
    pub id: EntryId,
    #[serde(rename = "_class")]
    pub class: EntryClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlDetails {
    pub protocol: EntryProtocol,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderParamSpecOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderParamSpec {
    pub name: String,
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression",
        skip_serializing_if = "expression::is_null"
    )]
    pub value: Expression,
    pub description: Option<String>,
    pub options: HeaderParamSpecOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathParamSpecOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathParamSpec {
    pub name: String,
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression",
        skip_serializing_if = "expression::is_null"
    )]
    pub value: Expression,
    pub description: Option<String>,
    pub options: PathParamSpecOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParamSpecOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParamSpec {
    pub name: String,
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression",
        skip_serializing_if = "expression::is_null"
    )]
    pub value: Expression,
    pub description: Option<String>,
    pub options: QueryParamSpecOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormDataParamValue {
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression"
    )]
    #[serde(rename = "text")]
    Text(Expression),

    #[serde(rename = "path")]
    Binary(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormDataParamSpecOptions {
    pub disabled: bool,
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormDataParamSpec {
    pub name: String,
    // TODO: Handling both text value and file upload
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/POST
    pub value: FormDataParamValue,
    pub description: Option<String>,
    pub options: FormDataParamSpecOptions,
}

#[derive(Debug, Clone)]
pub enum BodySpec {
    Text(Option<String>),
    Json(Option<JsonValue>),
    FormData(IndexMap<FormDataParamId, FormDataParamSpec>),
    XWwwFormUrlencoded(IndexMap<FormDataParamId, FormDataParamSpec>),
    Binary(Option<PathBuf>),
}

impl BodySpec {
    pub fn body_type(&self) -> &'static str {
        match self {
            BodySpec::Text(_) => "text",
            BodySpec::Json(_) => "json",
            BodySpec::FormData(_) => "form-data",
            BodySpec::XWwwFormUrlencoded(_) => "x-www-form-urlencoded",
            BodySpec::Binary(_) => "binary",
        }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        match self {
            BodySpec::Text(None) | BodySpec::Json(None) | BodySpec::Binary(None) => true,
            BodySpec::FormData(map) | BodySpec::XWwwFormUrlencoded(map) => map.is_empty(),
            _ => false,
        }
    }
}

/// Wrapper for HTTP body that handles HCL serialization with labeled blocks
#[derive(Debug, Clone)]
pub struct BodyBlock(BodySpec);

impl BodyBlock {
    pub fn new(spec: BodySpec) -> Self {
        BodyBlock(spec)
    }

    pub fn into_inner(self) -> BodySpec {
        self.0
    }
}

impl std::ops::Deref for BodyBlock {
    type Target = BodySpec;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for BodyBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;
        create_body_block(&self.0)
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BodyBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let block = hcl::structure::Block::deserialize(deserializer)?;
        let spec = deserialize_body_spec_from_block(&block).map_err(D::Error::custom)?;
        Ok(BodyBlock(spec))
    }
}

fn create_heredoc(content: &str) -> Result<hcl::expr::Heredoc, String> {
    use hcl::Identifier;

    const INDENT: &str = "  ";
    const DELIMITER: &str = "EOF";

    let indented = content
        .lines()
        .map(|l| format!("{INDENT}{l}"))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(hcl::expr::Heredoc::new(
        Identifier::new(DELIMITER).map_err(|e| e.to_string())?,
        indented,
    )
    .with_strip_mode(hcl::HeredocStripMode::Indent))
}

/// Parse heredoc content from HCL string representation
fn parse_heredoc_content(hcl_str: &str) -> Option<String> {
    // HCL heredoc format: <<-DELIM\n  content\nDELIM
    let lines: Vec<&str> = hcl_str.lines().collect();
    if lines.len() < 2 {
        return None;
    }

    // Extract content between first and last line
    let content_lines = &lines[1..lines.len() - 1];

    // Remove leading indentation (2 spaces)
    let content = content_lines
        .iter()
        .map(|line| {
            if line.starts_with("  ") {
                &line[2..]
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    Some(content)
}

/// Parse heredoc from expression string
#[allow(dead_code)]
fn parse_heredoc_from_expression(hcl_str: &str) -> Option<String> {
    let trimmed = hcl_str.trim();

    if trimmed.starts_with("<<") {
        // Parse heredoc content from serialized HCL
        parse_heredoc_content(hcl_str)
    } else if trimmed.starts_with('"') && trimmed.ends_with('"') {
        // Plain string
        Some(trimmed[1..trimmed.len() - 1].to_string())
    } else {
        // Return as is
        Some(hcl_str.to_string())
    }
}

/// Extract heredoc template content from debug string representation
/// Format: TemplateExpr(Heredoc(Heredoc { delimiter: Identifier(EOF), template: "content", strip: Indent }))
fn extract_heredoc_template_from_debug(debug_str: &str) -> Option<String> {
    // Find template field
    let start_marker = "template: \"";
    let start = debug_str.find(start_marker)?;
    let start_idx = start + start_marker.len();

    // Find the end marker - look for ", strip:" which follows the template
    let end_marker = "\", strip:";
    let end = debug_str[start_idx..].find(end_marker)?;

    let content = &debug_str[start_idx..start_idx + end];

    // Unescape the content
    let unescaped = content
        .replace("\\n", "\n")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\");

    // Remove trailing newline added by heredoc
    Some(unescaped.trim_end_matches('\n').to_string())
}

// TODO:
// I guess it's better to name it ResourceModel ?
// Ticket: https://mossland.atlassian.net/browse/SAPIC-533
#[derive(Debug, Clone)]
pub struct EntryModel {
    pub metadata: Block<EntryMetadataSpec>,
    pub url: Option<Block<UrlDetails>>,
    pub headers: Option<LabeledBlock<IndexMap<HeaderId, HeaderParamSpec>>>,
    pub path_params: Option<LabeledBlock<IndexMap<PathParamId, PathParamSpec>>>,
    pub query_params: Option<LabeledBlock<IndexMap<QueryParamId, QueryParamSpec>>>,
    pub body: Option<BodyBlock>,
}

impl Serialize for EntryModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use hcl::{Body, Identifier, Structure};
        use serde::ser::Error;

        // Build HCL body manually to have full control over structure
        let mut body_builder = Body::builder();

        // Serialize metadata as block - convert directly to HCL Block structure
        let metadata_value = hcl::to_value(&*self.metadata).map_err(S::Error::custom)?;
        if let hcl::Value::Object(obj) = metadata_value {
            let mut metadata_block = hcl::structure::Block::builder(
                Identifier::new("metadata").map_err(S::Error::custom)?,
            );
            for (key, val) in obj {
                metadata_block = metadata_block.add_attribute((key, val));
            }
            body_builder = body_builder.add_structure(Structure::Block(metadata_block.build()));
        }

        // Serialize url if present as block
        if let Some(url) = &self.url {
            let url_value = hcl::to_value(&**url).map_err(S::Error::custom)?;
            if let hcl::Value::Object(obj) = url_value {
                let mut url_block = hcl::structure::Block::builder(
                    Identifier::new("url").map_err(S::Error::custom)?,
                );
                for (key, val) in obj {
                    url_block = url_block.add_attribute((key, val));
                }
                body_builder = body_builder.add_structure(Structure::Block(url_block.build()));
            }
        }

        // Serialize headers if present - these are labeled blocks
        if let Some(headers) = &self.headers {
            for (id, spec) in (&**headers).iter() {
                // Serialize spec to HCL string, then parse back to extract attributes
                let spec_hcl = format!(
                    "temp {{\n{}\n}}",
                    hcl::to_string(spec).map_err(S::Error::custom)?
                );
                let temp_body: hcl::Body = hcl::from_str(&spec_hcl).map_err(S::Error::custom)?;
                let temp_block = temp_body
                    .blocks()
                    .next()
                    .ok_or_else(|| S::Error::custom("Failed to parse spec block"))?;

                let mut header_block = hcl::structure::Block::builder(
                    Identifier::new("header").map_err(S::Error::custom)?,
                )
                .add_label(hcl::structure::BlockLabel::String(id.to_string()));

                for attr in temp_block.body().attributes() {
                    header_block =
                        header_block.add_attribute((attr.key().to_string(), attr.expr().clone()));
                }

                body_builder = body_builder.add_structure(Structure::Block(header_block.build()));
            }
        }

        // Serialize path_params if present
        if let Some(path_params) = &self.path_params {
            for (id, spec) in (&**path_params).iter() {
                let spec_hcl = format!(
                    "temp {{\n{}\n}}",
                    hcl::to_string(spec).map_err(S::Error::custom)?
                );
                let temp_body: hcl::Body = hcl::from_str(&spec_hcl).map_err(S::Error::custom)?;
                let temp_block = temp_body
                    .blocks()
                    .next()
                    .ok_or_else(|| S::Error::custom("Failed to parse spec block"))?;

                let mut param_block = hcl::structure::Block::builder(
                    Identifier::new("path_param").map_err(S::Error::custom)?,
                )
                .add_label(hcl::structure::BlockLabel::String(id.to_string()));

                for attr in temp_block.body().attributes() {
                    param_block =
                        param_block.add_attribute((attr.key().to_string(), attr.expr().clone()));
                }

                body_builder = body_builder.add_structure(Structure::Block(param_block.build()));
            }
        }

        // Serialize query_params if present
        if let Some(query_params) = &self.query_params {
            for (id, spec) in (&**query_params).iter() {
                let spec_hcl = format!(
                    "temp {{\n{}\n}}",
                    hcl::to_string(spec).map_err(S::Error::custom)?
                );
                let temp_body: hcl::Body = hcl::from_str(&spec_hcl).map_err(S::Error::custom)?;
                let temp_block = temp_body
                    .blocks()
                    .next()
                    .ok_or_else(|| S::Error::custom("Failed to parse spec block"))?;

                let mut param_block = hcl::structure::Block::builder(
                    Identifier::new("query_param").map_err(S::Error::custom)?,
                )
                .add_label(hcl::structure::BlockLabel::String(id.to_string()));

                for attr in temp_block.body().attributes() {
                    param_block =
                        param_block.add_attribute((attr.key().to_string(), attr.expr().clone()));
                }

                body_builder = body_builder.add_structure(Structure::Block(param_block.build()));
            }
        }

        // Add body if present
        if let Some(body) = &self.body {
            let body_hcl_block = create_body_block(&body.0).map_err(S::Error::custom)?;
            body_builder = body_builder.add_structure(Structure::Block(body_hcl_block));
        }

        body_builder.build().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EntryModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use hcl::{Body, Structure, structure::BlockLabel};
        use indexmap::IndexMap;
        use serde::de::Error;

        let hcl_body = Body::deserialize(deserializer)?;

        let mut metadata = None;
        let mut url = None;
        let mut headers_map: IndexMap<HeaderId, HeaderParamSpec> = IndexMap::new();
        let mut path_params_map: IndexMap<PathParamId, PathParamSpec> = IndexMap::new();
        let mut query_params_map: IndexMap<QueryParamId, QueryParamSpec> = IndexMap::new();
        let mut body = None;

        // Parse structures and categorize them
        for structure in hcl_body.into_iter() {
            if let Structure::Block(block) = structure {
                let identifier = block.identifier().to_string();

                match identifier.as_str() {
                    "metadata" => {
                        let spec: EntryMetadataSpec = hcl::from_body(block.body().clone())
                            .map_err(|e| {
                                D::Error::custom(format!("Failed to parse metadata: {}", e))
                            })?;
                        metadata = Some(Block::new(spec));
                    }
                    "url" => {
                        let details: UrlDetails = hcl::from_body(block.body().clone())
                            .map_err(|e| D::Error::custom(format!("Failed to parse url: {}", e)))?;
                        url = Some(Block::new(details));
                    }
                    "header" => {
                        let id_str = block
                            .labels()
                            .first()
                            .ok_or_else(|| D::Error::custom("Header block missing ID label"))?;
                        let id_string = match id_str {
                            BlockLabel::String(s) => s.clone(),
                            BlockLabel::Identifier(i) => i.to_string(),
                        };
                        let id: HeaderId = serde_json::from_str(&format!("\"{}\"", id_string))
                            .map_err(|e| D::Error::custom(format!("Invalid header ID: {}", e)))?;

                        let spec: HeaderParamSpec =
                            hcl::from_body(block.body().clone()).map_err(|e| {
                                D::Error::custom(format!("Failed to parse header spec: {}", e))
                            })?;
                        headers_map.insert(id, spec);
                    }
                    "path_param" => {
                        let id_str = block
                            .labels()
                            .first()
                            .ok_or_else(|| D::Error::custom("Path param block missing ID label"))?;
                        let id_string = match id_str {
                            BlockLabel::String(s) => s.clone(),
                            BlockLabel::Identifier(i) => i.to_string(),
                        };
                        let id: PathParamId = serde_json::from_str(&format!("\"{}\"", id_string))
                            .map_err(|e| {
                            D::Error::custom(format!("Invalid path param ID: {}", e))
                        })?;

                        let spec: PathParamSpec =
                            hcl::from_body(block.body().clone()).map_err(|e| {
                                D::Error::custom(format!("Failed to parse path param spec: {}", e))
                            })?;
                        path_params_map.insert(id, spec);
                    }
                    "query_param" => {
                        let id_str = block.labels().first().ok_or_else(|| {
                            D::Error::custom("Query param block missing ID label")
                        })?;
                        let id_string = match id_str {
                            BlockLabel::String(s) => s.clone(),
                            BlockLabel::Identifier(i) => i.to_string(),
                        };
                        let id: QueryParamId = serde_json::from_str(&format!("\"{}\"", id_string))
                            .map_err(|e| {
                                D::Error::custom(format!("Invalid query param ID: {}", e))
                            })?;

                        let spec: QueryParamSpec =
                            hcl::from_body(block.body().clone()).map_err(|e| {
                                D::Error::custom(format!("Failed to parse query param spec: {}", e))
                            })?;
                        query_params_map.insert(id, spec);
                    }
                    "body" => {
                        let spec = deserialize_body_spec_from_block(&block).map_err(|e| {
                            D::Error::custom(format!("Failed to parse body: {}", e))
                        })?;
                        body = Some(BodyBlock::new(spec));
                    }
                    _ => {}
                }
            }
        }

        Ok(EntryModel {
            metadata: metadata.ok_or_else(|| D::Error::custom("Missing metadata block"))?,
            url,
            headers: if headers_map.is_empty() {
                None
            } else {
                Some(LabeledBlock::new(headers_map))
            },
            path_params: if path_params_map.is_empty() {
                None
            } else {
                Some(LabeledBlock::new(path_params_map))
            },
            query_params: if query_params_map.is_empty() {
                None
            } else {
                Some(LabeledBlock::new(query_params_map))
            },
            body,
        })
    }
}

fn create_body_block(body_spec: &BodySpec) -> Result<hcl::structure::Block, String> {
    use hcl::{Identifier, structure::Block as HclBlock};

    let label = body_spec.body_type();

    match body_spec {
        BodySpec::Text(content) => {
            if let Some(text) = content {
                let heredoc = create_heredoc(text)?;
                Ok(
                    HclBlock::builder(Identifier::new("body").map_err(|e| e.to_string())?)
                        .add_label(Identifier::new(label).map_err(|e| e.to_string())?)
                        .add_attribute(("value", heredoc))
                        .build(),
                )
            } else {
                Ok(
                    HclBlock::builder(Identifier::new("body").map_err(|e| e.to_string())?)
                        .add_label(Identifier::new(label).map_err(|e| e.to_string())?)
                        .build(),
                )
            }
        }
        BodySpec::Json(content) => {
            if let Some(json_val) = content {
                let json_str = serde_json::to_string_pretty(json_val).map_err(|e| e.to_string())?;
                let heredoc = create_heredoc(&json_str)?;
                Ok(
                    HclBlock::builder(Identifier::new("body").map_err(|e| e.to_string())?)
                        .add_label(Identifier::new(label).map_err(|e| e.to_string())?)
                        .add_attribute(("value", heredoc))
                        .build(),
                )
            } else {
                Ok(
                    HclBlock::builder(Identifier::new("body").map_err(|e| e.to_string())?)
                        .add_label(Identifier::new(label).map_err(|e| e.to_string())?)
                        .build(),
                )
            }
        }
        BodySpec::FormData(form_data) => {
            let mut builder =
                HclBlock::builder(Identifier::new("body").map_err(|e| e.to_string())?)
                    .add_label(Identifier::new(label).map_err(|e| e.to_string())?);

            if !form_data.is_empty() {
                for (id, spec) in form_data.iter() {
                    // Serialize spec to HCL string, then parse back to extract attributes
                    let spec_hcl = format!(
                        "temp {{\n{}\n}}",
                        hcl::to_string(spec).map_err(|e| e.to_string())?
                    );
                    let temp_body: hcl::Body =
                        hcl::from_str(&spec_hcl).map_err(|e| e.to_string())?;
                    let temp_block = temp_body
                        .blocks()
                        .next()
                        .ok_or_else(|| "Failed to parse spec block".to_string())?;

                    let mut nested_builder =
                        HclBlock::builder(Identifier::new("form-data").map_err(|e| e.to_string())?)
                            .add_label(hcl::structure::BlockLabel::String(id.to_string()));

                    for attr in temp_block.body().attributes() {
                        nested_builder = nested_builder
                            .add_attribute((attr.key().to_string(), attr.expr().clone()));
                    }

                    builder = builder.add_block(nested_builder.build());
                }
            }

            Ok(builder.build())
        }
        BodySpec::XWwwFormUrlencoded(form_data) => {
            let mut builder =
                HclBlock::builder(Identifier::new("body").map_err(|e| e.to_string())?)
                    .add_label(Identifier::new(label).map_err(|e| e.to_string())?);

            if !form_data.is_empty() {
                for (id, spec) in form_data.iter() {
                    // Serialize spec to HCL string, then parse back to extract attributes
                    let spec_hcl = format!(
                        "temp {{\n{}\n}}",
                        hcl::to_string(spec).map_err(|e| e.to_string())?
                    );
                    let temp_body: hcl::Body =
                        hcl::from_str(&spec_hcl).map_err(|e| e.to_string())?;
                    let temp_block = temp_body
                        .blocks()
                        .next()
                        .ok_or_else(|| "Failed to parse spec block".to_string())?;

                    let mut nested_builder = HclBlock::builder(
                        Identifier::new("x-www-form-urlencoded").map_err(|e| e.to_string())?,
                    )
                    .add_label(hcl::structure::BlockLabel::String(id.to_string()));

                    for attr in temp_block.body().attributes() {
                        nested_builder = nested_builder
                            .add_attribute((attr.key().to_string(), attr.expr().clone()));
                    }

                    builder = builder.add_block(nested_builder.build());
                }
            }

            Ok(builder.build())
        }
        BodySpec::Binary(path) => {
            if let Some(file_path) = path {
                Ok(
                    HclBlock::builder(Identifier::new("body").map_err(|e| e.to_string())?)
                        .add_label(Identifier::new(label).map_err(|e| e.to_string())?)
                        .add_attribute(("path", file_path.display().to_string()))
                        .build(),
                )
            } else {
                Ok(
                    HclBlock::builder(Identifier::new("body").map_err(|e| e.to_string())?)
                        .add_label(Identifier::new(label).map_err(|e| e.to_string())?)
                        .build(),
                )
            }
        }
    }
}

fn deserialize_body_spec_from_block(block: &hcl::structure::Block) -> Result<BodySpec, String> {
    #[derive(Deserialize)]
    struct BodyBlock {
        #[serde(default)]
        value: Option<hcl::Expression>,
        #[serde(default)]
        path: Option<PathBuf>,
        #[serde(rename = "form-data")]
        #[serde(default)]
        form_data: Option<LabeledBlock<IndexMap<FormDataParamId, FormDataParamSpec>>>,
        #[serde(rename = "x-www-form-urlencoded")]
        #[serde(default)]
        x_www_form_urlencoded: Option<LabeledBlock<IndexMap<FormDataParamId, FormDataParamSpec>>>,
    }

    use hcl::structure::BlockLabel;

    let labels = block.labels();
    let body_type_label = labels
        .first()
        .ok_or_else(|| "Body block missing type label".to_string())?;

    let body_type = match body_type_label {
        BlockLabel::String(s) => s.clone(),
        BlockLabel::Identifier(id) => id.to_string(),
    };

    let body_content: BodyBlock = hcl::from_body(block.body().clone())
        .map_err(|e| format!("Failed to deserialize body content: {}", e))?;

    match body_type.as_str() {
        "text" => {
            let text = body_content
                .value
                .and_then(|expr| extract_heredoc_template_from_debug(&format!("{:?}", expr)));
            Ok(BodySpec::Text(text))
        }
        "json" => {
            let json = body_content.value.and_then(|expr| {
                let text = extract_heredoc_template_from_debug(&format!("{:?}", expr))?;
                serde_json::from_str(&text).ok()
            });
            Ok(BodySpec::Json(json))
        }
        "form-data" => {
            let form_data = body_content
                .form_data
                .map(|lb| lb.into_inner())
                .unwrap_or_default();
            Ok(BodySpec::FormData(form_data))
        }
        "x-www-form-urlencoded" => {
            let params = body_content
                .x_www_form_urlencoded
                .map(|lb| lb.into_inner())
                .unwrap_or_default();
            Ok(BodySpec::XWwwFormUrlencoded(params))
        }
        "binary" => Ok(BodySpec::Binary(body_content.path)),
        _ => Err(format!("Unknown body type: {}", body_type)),
    }
}

impl From<(EntryId, EntryClass)> for EntryModel {
    fn from((id, class): (EntryId, EntryClass)) -> Self {
        Self {
            metadata: Block::new(EntryMetadataSpec { id, class }),
            url: None,
            headers: None,
            query_params: None,
            path_params: None,
            body: None,
        }
    }
}

impl EntryModel {
    pub fn id(&self) -> EntryId {
        self.metadata.id.clone()
    }

    pub fn class(&self) -> EntryClass {
        self.metadata.class.clone()
    }

    pub fn protocol(&self) -> Option<EntryProtocol> {
        self.url.as_ref().map(|url| url.protocol.clone())
    }
}

#[cfg(test)]
mod tests {
    use hcl::{Expression as HclExpression, ser::LabeledBlock};
    use indexmap::indexmap;

    use super::*;

    fn test_item() -> EntryModel {
        let mut model = EntryModel {
            metadata: Block::new(EntryMetadataSpec {
                id: EntryId::new(),
                class: EntryClass::Endpoint,
            }),
            url: Some(Block::new(UrlDetails {
                protocol: EntryProtocol::Get,
                raw: "https://example.com".to_string(),
            })),
            headers: Some(LabeledBlock::new(indexmap! {
                    HeaderId::new() => HeaderParamSpec {
                        name: "Content-Type".to_string(),
                        value: HclExpression::String("application/json".to_string()),
                        description: Some("The content type of the request".to_string()),
                        options: HeaderParamSpecOptions {
                            disabled: false,
                            propagate: true
                        },
                    },
                    HeaderId::new() => HeaderParamSpec {
                        name: "Accept".to_string(),
                        value: HclExpression::String("application/json, application/xml".to_string()),
                        description: Some("The accept type of the request".to_string()),
                        options: HeaderParamSpecOptions {
                            disabled: false,
                            propagate: true
                    },
                }
            })),
            path_params: Some(LabeledBlock::new(indexmap! {
                PathParamId::new() => PathParamSpec {
                    name: "path_param1".to_string(),
                    value: Expression::String("bar".to_string()),
                    description: None,
                    options: PathParamSpecOptions {
                        disabled: false,
                        propagate: true,
                    },
                }
            })),
            query_params: Some(LabeledBlock::new(indexmap! {
                QueryParamId::new() => QueryParamSpec {
                    name: "query_param1".to_string(),
                    value: HclExpression::String("foo".to_string()),
                    description: None,
                    options: QueryParamSpecOptions {
                        disabled: false,
                        propagate: true
                    }
                }
            })),
            body: None,
        };
        model.body = Some(BodyBlock::new(BodySpec::FormData(indexmap! {
            FormDataParamId::new() => FormDataParamSpec {
                name: "file".to_string(),
                value: FormDataParamValue::Binary(PathBuf::from("foo/bar.txt")),
                description: None,
                options: FormDataParamSpecOptions {
                    disabled: false,
                    propagate: false,
                },
            },
            FormDataParamId::new() => FormDataParamSpec {
                name: "text".to_string(),
                value: FormDataParamValue::Text(HclExpression::String("Test".to_string())),
                description: None,
                options: FormDataParamSpecOptions {
                    disabled: false,
                    propagate: false,
                },
            },
        })));

        let str = hcl::to_string(&model).unwrap();
        println!("\n=== HCL Output ===\n{}", str);

        // JSON serialization is not supported for hcl::structure::Block
        // let json = serde_json::to_string(&model).unwrap();
        // println!("\n=== JSON Output ===\n{}", json);

        let model = hcl::from_str::<EntryModel>(&str).unwrap();

        model
    }

    #[test]
    fn test_edit() {
        let model = test_item();
        let model_string = hcl::to_string(&model).unwrap();

        let model = hcl::from_str::<EntryModel>(&model_string).unwrap();
        dbg!(&model);
    }

    #[test]
    fn test_body_text() {
        let mut model = EntryModel {
            metadata: Block::new(EntryMetadataSpec {
                id: EntryId::new(),
                class: EntryClass::Endpoint,
            }),
            url: None,
            headers: None,
            path_params: None,
            query_params: None,
            body: None,
        };
        model.body = Some(BodyBlock::new(BodySpec::Text(Some(
            "Hello, World!".to_string(),
        ))));

        let hcl_str = hcl::to_string(&model).unwrap();
        println!("\n=== Text Body HCL ===\n{}", hcl_str);

        let parsed = hcl::from_str::<EntryModel>(&hcl_str).unwrap();
        match parsed.body.as_ref().map(|b| &**b) {
            Some(BodySpec::Text(Some(text))) => assert_eq!(text, "Hello, World!"),
            other => panic!("Expected Text body, got: {:?}", other),
        }
    }

    #[test]
    fn test_body_json() {
        use serde_json::json;

        let mut model = EntryModel {
            metadata: Block::new(EntryMetadataSpec {
                id: EntryId::new(),
                class: EntryClass::Endpoint,
            }),
            url: None,
            headers: None,
            path_params: None,
            query_params: None,
            body: None,
        };
        model.body = Some(BodyBlock::new(BodySpec::Json(Some(json!({
            "name": "John Doe",
            "age": 30,
            "email": "john@example.com"
        })))));

        let hcl_str = hcl::to_string(&model).unwrap();
        println!("\n=== JSON Body HCL ===\n{}", hcl_str);

        let parsed = hcl::from_str::<EntryModel>(&hcl_str).unwrap();
        match parsed.body.as_ref().map(|b| &**b) {
            Some(BodySpec::Json(Some(json_val))) => {
                assert_eq!(json_val["name"], "John Doe");
                assert_eq!(json_val["age"], 30);
            }
            _ => panic!("Expected JSON body"),
        }
    }

    #[test]
    fn test_body_empty_json() {
        let mut model = EntryModel {
            metadata: Block::new(EntryMetadataSpec {
                id: EntryId::new(),
                class: EntryClass::Endpoint,
            }),
            url: Some(Block::new(UrlDetails {
                protocol: EntryProtocol::Get,
                raw: "https://example.com".to_string(),
            })),
            headers: None,
            path_params: None,
            query_params: None,
            body: None,
        };
        model.body = Some(BodyBlock::new(BodySpec::Json(None)));

        let hcl_str = hcl::to_string(&model).unwrap();
        println!("\n=== Empty JSON Body HCL ===\n{}", hcl_str);
        assert!(hcl_str.contains(r#"body json {"#));

        let parsed = hcl::from_str::<EntryModel>(&hcl_str).unwrap();
        match parsed.body.as_ref().map(|b| &**b) {
            Some(BodySpec::Json(None)) => {
                // Success - empty body
            }
            _ => panic!("Expected empty JSON body"),
        }
    }

    #[test]
    fn test_body_form_data() {
        let mut model = EntryModel {
            metadata: Block::new(EntryMetadataSpec {
                id: EntryId::new(),
                class: EntryClass::Endpoint,
            }),
            url: None,
            headers: None,
            path_params: None,
            query_params: None,
            body: Some(BodyBlock::new(BodySpec::FormData(indexmap! {
                FormDataParamId::new() => FormDataParamSpec {
                    name: "file".to_string(),
                    value: FormDataParamValue::Binary(PathBuf::from("foo/bar.txt")),
                    description: None,
                    options: FormDataParamSpecOptions {
                        disabled: false,
                        propagate: false,
                    },
                },
                FormDataParamId::new() => FormDataParamSpec {
                    name: "text".to_string(),
                    value: FormDataParamValue::Text(HclExpression::String("Test".to_string())),
                    description: None,
                    options: FormDataParamSpecOptions {
                        disabled: false,
                        propagate: false,
                    },
                },
            }))),
        };

        let hcl_str = hcl::to_string(&model).unwrap();
        println!("\n=== FormData Body HCL ===\n{}", hcl_str);

        let parsed = hcl::from_str::<EntryModel>(&hcl_str).unwrap();
        match parsed.body.as_ref().map(|b| &**b) {
            Some(BodySpec::FormData(form_data)) => {
                assert_eq!(form_data.len(), 2);
            }
            _ => panic!("Expected FormData body"),
        }
    }

    #[test]
    fn test_body_empty_form_data() {
        let mut model = EntryModel {
            metadata: Block::new(EntryMetadataSpec {
                id: EntryId::new(),
                class: EntryClass::Endpoint,
            }),
            url: None,
            headers: None,
            path_params: None,
            query_params: None,
            body: None,
        };
        model.body = Some(BodyBlock::new(BodySpec::FormData(IndexMap::new())));

        let hcl_str = hcl::to_string(&model).unwrap();
        println!("\n=== Empty FormData Body HCL ===\n{}", hcl_str);
        assert!(hcl_str.contains(r#"body form-data {"#));

        let parsed = hcl::from_str::<EntryModel>(&hcl_str).unwrap();
        match parsed.body.as_ref().map(|b| &**b) {
            Some(BodySpec::FormData(form_data)) => {
                assert!(form_data.is_empty());
            }
            _ => panic!("Expected empty FormData body"),
        }
    }

    #[test]
    fn test_body_x_www_form_urlencoded() {
        let mut model = EntryModel {
            metadata: Block::new(EntryMetadataSpec {
                id: EntryId::new(),
                class: EntryClass::Endpoint,
            }),
            url: None,
            headers: None,
            path_params: None,
            query_params: None,
            body: Some(BodyBlock::new(BodySpec::XWwwFormUrlencoded(indexmap! {
                FormDataParamId::new() => FormDataParamSpec {
                    name: "username".to_string(),
                    value: FormDataParamValue::Text(HclExpression::String("john_doe".to_string())),
                    description: None,
                    options: FormDataParamSpecOptions {
                        disabled: false,
                        propagate: false,
                    },
                },
                FormDataParamId::new() => FormDataParamSpec {
                    name: "password".to_string(),
                    value: FormDataParamValue::Text(HclExpression::String("secret123".to_string())),
                    description: None,
                    options: FormDataParamSpecOptions {
                        disabled: false,
                        propagate: false,
                    },
                },
            }))),
        };

        let hcl_str = hcl::to_string(&model).unwrap();
        println!("\n=== X-WWW-Form-Urlencoded Body HCL ===\n{}", hcl_str);
        assert!(hcl_str.contains(r#"body x-www-form-urlencoded {"#));
        assert!(hcl_str.contains(r#"x-www-form-urlencoded "#));

        let parsed = hcl::from_str::<EntryModel>(&hcl_str).unwrap();
        match parsed.body.as_ref().map(|b| &**b) {
            Some(BodySpec::XWwwFormUrlencoded(params)) => {
                assert_eq!(params.len(), 2);
            }
            _ => panic!("Expected XWwwFormUrlencoded body"),
        }
    }

    #[test]
    fn test_body_binary() {
        let mut model = EntryModel {
            metadata: Block::new(EntryMetadataSpec {
                id: EntryId::new(),
                class: EntryClass::Endpoint,
            }),
            url: None,
            headers: None,
            path_params: None,
            query_params: None,
            body: Some(BodyBlock::new(BodySpec::Binary(Some(PathBuf::from(
                "/path/to/file.bin",
            ))))),
        };

        let hcl_str = hcl::to_string(&model).unwrap();
        println!("\n=== Binary Body HCL ===\n{}", hcl_str);

        let parsed = hcl::from_str::<EntryModel>(&hcl_str).unwrap();
        match parsed.body.as_ref().map(|b| &**b) {
            Some(BodySpec::Binary(Some(path))) => {
                assert_eq!(path.to_str().unwrap(), "/path/to/file.bin");
            }
            _ => panic!("Expected Binary body"),
        }
    }
}
