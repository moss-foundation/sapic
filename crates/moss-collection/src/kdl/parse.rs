use anyhow::Result;
use kdl::{KdlDocument, KdlNode};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;

use crate::{
    kdl::{
        body::{FormDataBodyItem, FormDataOptions, UrlEncodedBodyItem, UrlEncodedOptions},
        foundations::{
            body::RequestBodyBlock,
            http::{
                HeaderParamBody, HeaderParamOptions, HttpRequestFile, PathParamBody,
                PathParamOptions, QueryParamBody, QueryParamOptions, UrlBlock,
            },
        },
        tokens::*,
    },
    models::types::{FormDataValue, RawBodyType},
};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("A `params` node is missing type")]
    MissingParamsType,
    #[error("A `params` node has invalid type")]
    InvalidParamsType,
    #[error("A `body` node is missing type")]
    MissingBodyType,
    #[error("A `body` node has invalid type")]
    InvalidBodyType,
    #[error("A `body` node is empty")]
    EmptyBody,
    #[error("A `body` node is ill-formatted")]
    IllFormattedBody,
    #[error("A `body` node has invalid `{typ}` content")]
    InvalidBodyContent { typ: String },
    #[error("A form data `body` node has invalid value type")]
    InvalidFormDataType,
    #[error("A binary `body` node has missing file path")]
    MissingBinaryPath,
    #[error("A binary `body` node has invalid file path")]
    InvalidBinaryPath,
}

pub struct ParseOptions {
    html_parse_mode: HtmlParseMode,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            html_parse_mode: HtmlParseMode::Strict,
        }
    }
}

#[derive(PartialEq)]
pub enum HtmlParseMode {
    /// Return an error when parsing html content with errors
    Strict,
    /// Ignore the errors
    Relaxed,
}

#[macro_export]
macro_rules! kdl_get_child {
    ( $kdl_node:expr, $child_name:expr ) => {
        $kdl_node.children().and_then(|c| c.get($child_name))
    };
}
#[macro_export]
macro_rules! kdl_get_arg_as_value {
    ( $kdl_document:expr, $arg_name:expr ) => {
        $kdl_document
            .get_arg($arg_name)
            .map(|value| value.to_owned())
    };
}

#[macro_export]
macro_rules! kdl_get_arg_as_integer {
    ( $kdl_document:expr, $arg_name:expr) => {
        kdl_get_arg_as_helper!($kdl_document, $arg_name, as_integer)
    };
}

#[macro_export]
macro_rules! kdl_get_arg_as_string {
    ( $kdl_document:expr, $arg_name:expr) => {
        kdl_get_arg_as_value!($kdl_document, $arg_name).and_then(|value| {
            if value.is_string() {
                value.as_string().map(|s| s.to_string())
            } else {
                Some(value.to_string())
            }
        })
    };
}

#[macro_export]
macro_rules! kdl_get_arg_as_bool {
    ( $kdl_document:expr, $arg_name:expr) => {
        kdl_get_arg_as_helper!($kdl_document, $arg_name, as_bool)
    };
}

macro_rules! kdl_get_arg_as_helper {
    ($kdl_document:expr, $arg_name:expr, $conversion:ident) => {
        $kdl_document
            .get_arg($arg_name)
            .and_then(|kdl_value| kdl_value.$conversion())
    };
}

fn parse_url_node(node: &KdlNode) -> Result<UrlBlock> {
    if let Some(document) = node.children() {
        Ok(UrlBlock {
            raw: kdl_get_arg_as_string!(document, "raw"),
            host: kdl_get_arg_as_string!(document, "host"),
        })
    } else {
        Ok(UrlBlock {
            raw: None,
            host: None,
        })
    }
}

fn parse_query_params(node: &KdlNode) -> Result<HashMap<String, QueryParamBody>> {
    let mut params: HashMap<String, QueryParamBody> = HashMap::new();
    if let Some(document) = node.children() {
        for param_node in document.nodes() {
            let name = param_node.name().to_string();
            let param_body = parse_query_param_body(param_node)?;
            params.insert(name, param_body);
        }
    }
    Ok(params)
}

fn parse_query_param_body(node: &KdlNode) -> Result<QueryParamBody> {
    if let Some(fields) = node.children() {
        let value = kdl_get_arg_as_string!(fields, "value");
        let desc = kdl_get_arg_as_string!(fields, "desc");
        let order = kdl_get_arg_as_integer!(fields, "order").and_then(|value| Some(value as usize));
        let disabled = kdl_get_arg_as_bool!(fields, "disabled").unwrap_or(false);
        let options_node = fields.get("options");
        let options = if let Some(options_node) = options_node {
            parse_query_param_options(options_node)?
        } else {
            QueryParamOptions::default()
        };
        Ok(QueryParamBody {
            value: value.unwrap_or("".to_string()),
            desc,
            order,
            disabled,
            options,
        })
    } else {
        Ok(QueryParamBody::default())
    }
}

fn parse_query_param_options(node: &KdlNode) -> Result<QueryParamOptions> {
    if let Some(fields) = node.children() {
        let propagate = kdl_get_arg_as_bool!(fields, "propagate").unwrap_or(false);
        Ok(QueryParamOptions { propagate })
    } else {
        Ok(QueryParamOptions::default())
    }
}

fn parse_path_params(node: &KdlNode) -> Result<HashMap<String, PathParamBody>> {
    let mut params: HashMap<String, PathParamBody> = HashMap::new();
    if let Some(document) = node.children() {
        for param_node in document.nodes() {
            let name = param_node.name().to_string();
            let param_body = parse_path_param_body(param_node)?;
            params.insert(name, param_body);
        }
    }
    Ok(params)
}

fn parse_path_param_body(node: &KdlNode) -> Result<PathParamBody> {
    if let Some(fields) = node.children() {
        let value = kdl_get_arg_as_string!(fields, "value");
        let desc = kdl_get_arg_as_string!(fields, "desc");
        let order = kdl_get_arg_as_integer!(fields, "order").and_then(|value| Some(value as usize));
        let disabled = kdl_get_arg_as_bool!(fields, "disabled").unwrap_or(false);
        let options_node = fields.get("options");
        let options = if let Some(options_node) = options_node {
            parse_path_param_options(options_node)?
        } else {
            PathParamOptions::default()
        };
        Ok(PathParamBody {
            value: value.unwrap_or("".to_string()),
            desc,
            order,
            disabled,
            options,
        })
    } else {
        Ok(PathParamBody::default())
    }
}

fn parse_path_param_options(node: &KdlNode) -> Result<PathParamOptions> {
    if let Some(fields) = node.children() {
        let propagate = kdl_get_arg_as_bool!(fields, "propagate").unwrap_or(false);
        Ok(PathParamOptions { propagate })
    } else {
        Ok(PathParamOptions::default())
    }
}

fn parse_header_params(node: &KdlNode) -> Result<HashMap<String, HeaderParamBody>> {
    let mut headers: HashMap<String, HeaderParamBody> = HashMap::new();
    if let Some(document) = node.children() {
        for header_node in document.nodes() {
            let name = header_node.name().to_string();
            let header_body = parse_header_param_body(header_node)?;
            headers.insert(name, header_body);
        }
    }
    Ok(headers)
}

fn parse_header_param_body(node: &KdlNode) -> Result<HeaderParamBody> {
    if let Some(fields) = node.children() {
        let value = kdl_get_arg_as_string!(fields, "value");
        let desc = kdl_get_arg_as_string!(fields, "desc");
        let order = kdl_get_arg_as_integer!(fields, "order").and_then(|value| Some(value as usize));
        let disabled = kdl_get_arg_as_bool!(fields, "disabled").unwrap_or(false);
        let options_node = fields.get("options");
        let options = if let Some(options_node) = options_node {
            parse_header_param_options(options_node)?
        } else {
            HeaderParamOptions::default()
        };
        Ok(HeaderParamBody {
            value: value.unwrap_or("".to_string()),
            desc,
            order,
            disabled,
            options,
        })
    } else {
        Ok(HeaderParamBody::default())
    }
}

fn parse_header_param_options(node: &KdlNode) -> Result<HeaderParamOptions> {
    if let Some(fields) = node.children() {
        let propagate = kdl_get_arg_as_bool!(fields, "propagate").unwrap_or(false);
        Ok(HeaderParamOptions { propagate })
    } else {
        Ok(HeaderParamOptions::default())
    }
}

fn parse_body_node(node: &KdlNode, opts: &ParseOptions) -> Result<RequestBodyBlock> {
    let typ = node
        .get("type")
        .ok_or_else(|| ParseError::MissingBodyType)
        .map(|value| value.as_string())?
        // If the type's value is not a string
        .ok_or_else(|| ParseError::InvalidBodyType)?;
    match typ {
        BODY_TYPE_TEXT => Ok(RequestBodyBlock::Raw(parse_raw_body_text(&node)?)),
        BODY_TYPE_JSON => Ok(RequestBodyBlock::Raw(parse_raw_body_json(&node)?)),
        BODY_TYPE_HTML => Ok(RequestBodyBlock::Raw(parse_raw_body_html(
            &node,
            &opts.html_parse_mode,
        )?)),
        BODY_TYPE_XML => Ok(RequestBodyBlock::Raw(parse_raw_body_xml(&node)?)),
        BODY_TYPE_FORM_DATA => Ok(RequestBodyBlock::FormData(parse_form_data_params(&node)?)),
        BODY_TYPE_URLENCODED => Ok(RequestBodyBlock::UrlEncoded(parse_urlencoded_params(
            &node,
        )?)),
        BODY_TYPE_BINARY => Ok(RequestBodyBlock::Binary(parse_binary_path(&node)?)),
        _ => Err(ParseError::InvalidBodyType.into()),
    }
}

fn parse_raw_body_text(node: &KdlNode) -> Result<RawBodyType> {
    Ok(RawBodyType::Text(parse_raw_body_content(&node)?))
}

fn parse_raw_body_json(node: &KdlNode) -> Result<RawBodyType> {
    // Validate if the content is valid json
    let json_string = parse_raw_body_content(&node)?;
    let _: JsonValue =
        serde_json::from_str(json_string.as_str()).map_err(|_| ParseError::InvalidBodyContent {
            typ: "JSON".to_string(),
        })?;
    Ok(RawBodyType::Json(json_string))
}

fn parse_raw_body_html(node: &KdlNode, html_parse_mode: &HtmlParseMode) -> Result<RawBodyType> {
    // Validate if the content is valid html
    let html_string = parse_raw_body_content(&node)?;
    let parsed = scraper::Html::parse_document(&html_string);
    if html_parse_mode == &HtmlParseMode::Strict && !parsed.errors.is_empty() {
        dbg!(parsed.errors);
        return Err(ParseError::InvalidBodyContent {
            typ: "HTML".to_string(),
        }
        .into());
    }
    Ok(RawBodyType::Html(html_string))
}

fn parse_raw_body_xml(node: &KdlNode) -> Result<RawBodyType> {
    // Validate if the content is valid xml
    let xml_sting = parse_raw_body_content(&node)?;
    let parser = xml::reader::EventReader::from_str(&xml_sting);
    for event in parser {
        if event.is_err() {
            return Err(ParseError::InvalidBodyContent {
                typ: "XML".to_string(),
            }
            .into());
        }
    }
    Ok(RawBodyType::Xml(xml_sting))
}

fn parse_raw_body_content(node: &KdlNode) -> Result<String> {
    let raw_string = node
        .children()
        .ok_or(ParseError::EmptyBody)?
        .nodes()
        .into_iter()
        .next()
        .ok_or(ParseError::EmptyBody)?
        .name()
        .to_string();

    let result = raw_string
        .lines()
        .filter(|line| line.trim() != RAW_STRING_PREFIX && line.trim() != RAW_STRING_SUFFIX)
        .map(|line| {
            line.strip_prefix(RAW_STRING_INDENT)
                .ok_or(ParseError::IllFormattedBody.into())
        })
        .collect::<Result<Vec<_>>>()?
        .join("\n");

    Ok(result)
}

fn parse_form_data_params(node: &KdlNode) -> Result<HashMap<String, FormDataBodyItem>> {
    let mut params: HashMap<String, FormDataBodyItem> = HashMap::new();
    if let Some(document) = node.children() {
        for param_node in document.nodes() {
            let key = param_node.name().to_string();
            let param_body = parse_form_data_body(param_node)?;
            params.insert(key, param_body);
        }
    }
    Ok(params)
}

fn parse_form_data_body(node: &KdlNode) -> Result<FormDataBodyItem> {
    if let Some(fields) = node.children() {
        // FIXME: We now default to Text("") if we can't find type and value from the file
        // Should we raise an error here?
        let typ = kdl_get_arg_as_string!(fields, "type").unwrap_or("text".to_string());
        let value = kdl_get_arg_as_string!(fields, "value").unwrap_or_default();
        let desc = kdl_get_arg_as_string!(fields, "desc");
        let order = kdl_get_arg_as_integer!(fields, "order").and_then(|value| Some(value as usize));
        let disabled = kdl_get_arg_as_bool!(fields, "disabled").unwrap_or(false);
        let options_node = fields.get("options");
        let options = if let Some(options_node) = options_node {
            parse_form_data_options(options_node)?
        } else {
            FormDataOptions::default()
        };

        Ok(FormDataBodyItem {
            value: match typ.as_ref() {
                "text" => FormDataValue::Text(value.to_string()),
                "file" => FormDataValue::File(PathBuf::from(value)),
                _ => return Err(ParseError::InvalidFormDataType.into()),
            },
            desc,
            order,
            disabled,
            options,
        })
    } else {
        Ok(FormDataBodyItem::default())
    }
}

fn parse_form_data_options(node: &KdlNode) -> Result<FormDataOptions> {
    if let Some(fields) = node.children() {
        let propagate = kdl_get_arg_as_bool!(fields, "propagate").unwrap_or(false);
        Ok(FormDataOptions { propagate })
    } else {
        Ok(FormDataOptions::default())
    }
}

fn parse_urlencoded_params(node: &KdlNode) -> Result<HashMap<String, UrlEncodedBodyItem>> {
    let mut params: HashMap<String, UrlEncodedBodyItem> = HashMap::new();
    if let Some(document) = node.children() {
        for param_node in document.nodes() {
            let key = param_node.name().to_string();
            let param_body = parse_urlencoded_body(param_node)?;
            params.insert(key, param_body);
        }
    }
    Ok(params)
}

fn parse_urlencoded_body(node: &KdlNode) -> Result<UrlEncodedBodyItem> {
    if let Some(fields) = node.children() {
        let value = kdl_get_arg_as_string!(fields, "value").unwrap_or_default();
        let desc = kdl_get_arg_as_string!(fields, "desc");
        let order = kdl_get_arg_as_integer!(fields, "order").and_then(|value| Some(value as usize));
        let disabled = kdl_get_arg_as_bool!(fields, "disabled").unwrap_or(false);
        let options_node = fields.get("options");
        let options = if let Some(options_node) = options_node {
            parse_urlencoded_options(options_node)?
        } else {
            UrlEncodedOptions::default()
        };

        Ok(UrlEncodedBodyItem {
            value,
            desc,
            order,
            disabled,
            options,
        })
    } else {
        Ok(UrlEncodedBodyItem::default())
    }
}

fn parse_urlencoded_options(node: &KdlNode) -> Result<UrlEncodedOptions> {
    if let Some(fields) = node.children() {
        let propagate = kdl_get_arg_as_bool!(fields, "propagate").unwrap_or(false);
        Ok(UrlEncodedOptions { propagate })
    } else {
        Ok(UrlEncodedOptions::default())
    }
}

fn parse_binary_path(node: &KdlNode) -> Result<PathBuf> {
    let path = node
        .get("path")
        .ok_or(ParseError::MissingBinaryPath)?
        .as_string()
        .ok_or(ParseError::InvalidBinaryPath)?;
    Ok(PathBuf::from(path))
}

pub fn parse(input: &str, opts: &ParseOptions) -> Result<HttpRequestFile> {
    let document: KdlDocument = input.parse()?;
    let mut request = HttpRequestFile::default();

    for node in document {
        match node.name().to_string().as_str() {
            URL_LIT => {
                request.url = parse_url_node(&node)?;
            }
            PARAMS_LIT => {
                // FIXME: Should we handle duplicate query/path param nodes?
                let typ = node
                    .get("type")
                    .ok_or_else(|| ParseError::MissingParamsType)
                    .map(|value| value.as_string())?
                    // If the type's value is not a string
                    .ok_or_else(|| ParseError::InvalidParamsType)?;

                match typ {
                    QUERY_LIT => {
                        request.query_params = parse_query_params(&node)?;
                    }
                    PATH_LIT => {
                        request.path_params = parse_path_params(&node)?;
                    }
                    _ => return Err(ParseError::InvalidParamsType.into()),
                }
            }
            BODY_LIT => {
                request.body = Some(parse_body_node(&node, opts)?);
            }
            HEADERS_LIT => {
                request.headers = parse_header_params(&node)?;
            }

            _ => {}
        }
    }

    Ok(request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kdl::foundations::{
        body::RequestBodyBlock,
        http::{QueryParamBody, QueryParamOptions, UrlBlock},
    };
    use kdl::KdlNode;

    use std::fs;

    #[test]
    fn parse_url_node_empty() {
        let text = r#"url {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let url_node = parse_url_node(&node).unwrap();
        assert_eq!(
            url_node,
            UrlBlock {
                raw: None,
                host: None
            }
        );
        assert_eq!(url_node, UrlBlock::default())
    }

    #[test]
    fn parse_url_node_incomplete() {
        let text = r#"url {
    raw "raw"
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let url_node = parse_url_node(&node).unwrap();
        assert_eq!(
            url_node,
            UrlBlock {
                raw: Some("raw".to_string()),
                host: None
            }
        );
    }

    #[test]
    fn parse_url_node_normal() {
        let text = r#"url {
    raw "{{baseUrl}}/objects"
    host "{{baseUrl}}"
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let url_node = parse_url_node(&node).unwrap();
        assert_eq!(
            url_node,
            UrlBlock {
                raw: Some("{{baseUrl}}/objects".to_string()),
                host: Some("{{baseUrl}}".to_string())
            }
        )
    }

    #[test]
    fn parse_query_param_body_empty() {
        let text = r#"param {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_body = parse_query_param_body(&node).unwrap();
        assert_eq!(
            param_body,
            QueryParamBody {
                value: "".to_string(),
                desc: None,
                order: None,
                disabled: false,
                options: Default::default(),
            }
        )
    }

    #[test]
    fn parse_query_param_body_numeric_value() {
        let text = r#"param {
    value 1
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_body = parse_query_param_body(&node).unwrap();
        assert_eq!(param_body.value, "1".to_string());
    }

    #[test]
    fn parse_query_param_body_full() {
        let text = r#"param {
    value "value"
    desc "desc"
    order 1
    disabled #true
    options {
        propagate #true
    }
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_body = parse_query_param_body(&node).unwrap();
        assert_eq!(
            param_body,
            QueryParamBody {
                value: "value".to_string(),
                desc: Some("desc".to_string()),
                order: Some(1),
                disabled: true,
                options: QueryParamOptions { propagate: true }
            }
        )
    }

    #[test]
    fn parse_query_param_options_empty() {
        let text = r#"options {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_query_param_options(&node).unwrap();
        assert_eq!(param_options, QueryParamOptions::default())
    }

    #[test]
    fn parse_query_param_options_normal() {
        let text = r#"options {
    propagate #true
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_query_param_options(&node).unwrap();
        assert_eq!(param_options, QueryParamOptions { propagate: true })
    }

    #[test]
    fn parse_query_params_empty() {
        let text = r#"params type=query {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let query_params = parse_query_params(&node).unwrap();
        assert!(query_params.is_empty());
    }

    #[test]
    fn parse_query_params_one_param() {
        let text = r#"params type=query {
    pageToken {}
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let query_params = parse_query_params(&node).unwrap();
        assert_eq!(query_params.len(), 1);
        assert_eq!(query_params["pageToken"], QueryParamBody::default());
        let text = r#"params type=query {
    visibleOnly {
        value "true"
        desc "desc"
        disabled #true
        order 2
    }
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let query_params = parse_query_params(&node).unwrap();
        assert_eq!(query_params.len(), 1);
        assert_eq!(
            query_params["visibleOnly"],
            QueryParamBody {
                value: "true".to_string(),
                desc: Some("desc".to_string()),
                order: Some(2),
                disabled: true,
                options: Default::default(),
            }
        );
    }

    #[test]
    fn parse_query_params_two_params() {
        let text = r#"params type=query {
    param1 {
        value "1"
    }

    param2 {
        value "2"
    }
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let query_params = parse_query_params(&node).unwrap();
        assert_eq!(query_params.len(), 2);
        assert_eq!(
            query_params["param1"],
            QueryParamBody {
                value: "1".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            query_params["param2"],
            QueryParamBody {
                value: "2".to_string(),
                ..Default::default()
            }
        );
    }

    // TODO: Path
    #[test]
    fn parse_path_param_body_empty() {
        let text = r#"param {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_body = parse_path_param_body(&node).unwrap();
        assert_eq!(
            param_body,
            PathParamBody {
                value: "".to_string(),
                desc: None,
                order: None,
                disabled: false,
                options: Default::default(),
            }
        )
    }

    #[test]
    fn parse_path_param_body_numeric_value() {
        let text = r#"param {
    value 1
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_body = parse_path_param_body(&node).unwrap();
        assert_eq!(param_body.value, "1".to_string());
    }

    #[test]
    fn parse_path_param_body_full() {
        let text = r#"param {
    value "value"
    desc "desc"
    order 1
    disabled #true
    options {
        propagate #true
    }
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_body = parse_path_param_body(&node).unwrap();
        assert_eq!(
            param_body,
            PathParamBody {
                value: "value".to_string(),
                desc: Some("desc".to_string()),
                order: Some(1),
                disabled: true,
                options: PathParamOptions { propagate: true }
            }
        )
    }

    #[test]
    fn parse_path_param_options_empty() {
        let text = r#"options {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_path_param_options(&node).unwrap();
        assert_eq!(param_options, PathParamOptions::default())
    }

    #[test]
    fn parse_path_param_options_normal() {
        let text = r#"options {
    propagate #true
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_path_param_options(&node).unwrap();
        assert_eq!(param_options, PathParamOptions { propagate: true })
    }

    #[test]
    fn parse_path_params_empty() {
        let text = r#"params type=path {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let path_params = parse_path_params(&node).unwrap();
        assert!(path_params.is_empty());
    }

    #[test]
    fn parse_path_params_one_param() {
        let text = r#"params type=path {
    pageToken {}
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let path_params = parse_path_params(&node).unwrap();
        assert_eq!(path_params.len(), 1);
        assert_eq!(path_params["pageToken"], PathParamBody::default());
        let text = r#"params type=path {
    visibleOnly {
        value "true"
        desc "desc"
        disabled #true
        order 2
    }
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let path_params = parse_path_params(&node).unwrap();
        assert_eq!(path_params.len(), 1);
        assert_eq!(
            path_params["visibleOnly"],
            PathParamBody {
                value: "true".to_string(),
                desc: Some("desc".to_string()),
                order: Some(2),
                disabled: true,
                options: Default::default(),
            }
        );
    }

    #[test]
    fn parse_path_params_two_params() {
        let text = r#"params type=path {
    param1 {
        value "1"
    }

    param2 {
        value "2"
    }
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let path_params = parse_path_params(&node).unwrap();
        assert_eq!(path_params.len(), 2);
        assert_eq!(
            path_params["param1"],
            PathParamBody {
                value: "1".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            path_params["param2"],
            PathParamBody {
                value: "2".to_string(),
                ..Default::default()
            }
        );
    }

    // TODO: headers
    #[test]
    fn parse_header_body_empty() {
        let text = r#"header {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let header_body = parse_header_param_body(&node).unwrap();
        assert_eq!(
            header_body,
            HeaderParamBody {
                value: "".to_string(),
                desc: None,
                order: None,
                disabled: false,
                options: Default::default(),
            }
        )
    }

    #[test]
    fn parse_header_body_numeric_value() {
        let text = r#"header {
    value 1
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let header_body = parse_header_param_body(&node).unwrap();
        assert_eq!(header_body.value, "1".to_string());
    }

    #[test]
    fn parse_header_body_full() {
        let text = r#"header {
    value "value"
    desc "desc"
    order 1
    disabled #true
    options {
        propagate #true
    }
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let header_body = parse_header_param_body(&node).unwrap();
        assert_eq!(
            header_body,
            HeaderParamBody {
                value: "value".to_string(),
                desc: Some("desc".to_string()),
                order: Some(1),
                disabled: true,
                options: HeaderParamOptions { propagate: true }
            }
        )
    }

    #[test]
    fn parse_header_param_options_empty() {
        let text = r#"options {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_header_param_options(&node).unwrap();
        assert_eq!(param_options, HeaderParamOptions::default())
    }

    #[test]
    fn parse_header_param_options_normal() {
        let text = r#"options {
    propagate #true
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_header_param_options(&node).unwrap();
        assert_eq!(param_options, HeaderParamOptions { propagate: true })
    }

    #[test]
    fn parse_header_params_empty() {
        let text = r#"headers {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let header_params = parse_header_params(&node).unwrap();
        assert!(header_params.is_empty());
    }

    #[test]
    fn parse_header_params_one_param() {
        let text = r#"headers {
    pageToken {}
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let header_params = parse_header_params(&node).unwrap();
        assert_eq!(header_params.len(), 1);
        assert_eq!(header_params["pageToken"], HeaderParamBody::default());
        let text = r#"headers {
    visibleOnly {
        value "true"
        desc "desc"
        disabled #true
        order 2
    }
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let header_params = parse_header_params(&node).unwrap();
        assert_eq!(header_params.len(), 1);
        assert_eq!(
            header_params["visibleOnly"],
            HeaderParamBody {
                value: "true".to_string(),
                desc: Some("desc".to_string()),
                order: Some(2),
                disabled: true,
                options: Default::default(),
            }
        );
    }

    #[test]
    fn parse_header_params_two_params() {
        let text = r#"headers {
            param1 {
                value "1"
            }

            param2 {
                value "2"
            }
        }"#;
        let node = KdlNode::parse(&text).unwrap();
        let header_params = parse_header_params(&node).unwrap();
        assert_eq!(header_params.len(), 2);
        assert_eq!(
            header_params["param1"],
            HeaderParamBody {
                value: "1".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            header_params["param2"],
            HeaderParamBody {
                value: "2".to_string(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn parse_body_text() {
        let text = r###"body type=text {
    #"""
    A test string
    """#
}"###;
        let request = parse(&text, &ParseOptions::default()).unwrap();
        assert_eq!(
            request.body.unwrap(),
            RequestBodyBlock::Raw(RawBodyType::Text("A test string".to_string()))
        );
    }

    #[test]
    fn parse_body_json() {
        let text = r###"body type=json {
    #"""
    {
        "key": "value",
        "object": {
            "inner": "value"
        }
    }
    """#
}"###;
        println!("{}", text);
        let json = r#"{
    "key": "value",
    "object": {
        "inner": "value"
    }
}"#;
        let request = parse(text, &ParseOptions::default()).unwrap();
        assert_eq!(
            request.body.unwrap(),
            RequestBodyBlock::Raw(RawBodyType::Json(json.to_string()))
        );
    }

    #[test]
    fn parse_body_json_invalid() {
        let text = r###"body type=json {
    #"""
    {
    """#
}"###;
        assert!(parse(text, &ParseOptions::default()).is_err());
    }

    // FIXME: Right now, an error will occur if the html does not contain `<!DOCTYPE>` delcaration
    // However, this is not necessarily an error, and should probably be separated from other errors in parsing
    #[test]
    fn parse_body_html() {
        let text = r###"body type=html {
    #"""
    <!DOCTYPE html>
    <head>
        <meta charset="UTF-8" />
        <title>Hello World!</title>
    </head>
    """#
}"###;
        let html = r###"<!DOCTYPE html>
<head>
    <meta charset="UTF-8" />
    <title>Hello World!</title>
</head>"###;
        let request = parse(text, &ParseOptions::default()).unwrap();
        assert_eq!(
            request.body.unwrap(),
            RequestBodyBlock::Raw(RawBodyType::Html(html.to_string()))
        );
    }

    #[test]
    fn parse_body_html_with_options() {
        let malformed = r###"body type=html {
    #"""
    <head>
        <meta charset="utf-8">
    """#
}"###;
        let html = r#"<head>
    <meta charset="utf-8">"#;
        assert!(
            parse(
                &malformed,
                &ParseOptions {
                    html_parse_mode: HtmlParseMode::Strict
                }
            )
            .is_err()
        );
        let request = parse(
            &malformed,
            &ParseOptions {
                html_parse_mode: HtmlParseMode::Relaxed,
            },
        )
        .unwrap();
        assert_eq!(
            request.body.unwrap(),
            RequestBodyBlock::Raw(RawBodyType::Html(html.to_string()))
        );
    }

    #[test]
    fn parse_body_xml() {
        let text = r###"body type=xml {
    #"""
    <note>
        <to>Tove</to>
        <from>Jani</from>
        <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
    </note>
    """#
}"###;
        let xml = r###"<note>
    <to>Tove</to>
    <from>Jani</from>
    <heading>Reminder</heading>
    <body>Don't forget me this weekend!</body>
</note>"###;
        let request = parse(&text, &ParseOptions::default()).unwrap();
        assert_eq!(
            request.body.unwrap(),
            RequestBodyBlock::Raw(RawBodyType::Xml(xml.to_string()))
        )
    }

    #[test]
    fn parse_body_xml_invalid() {
        let text = r###"body type=xml {
    #"""
    <note>
        <inner>1</inner>
    </NOTE>
    """#
}"###;
        assert!(parse(text, &ParseOptions::default()).is_err());
    }
    #[test]
    fn parse_body_form_data_empty() {
        let text = r###"body type=form-data {}"###;
        let request = parse(text, &ParseOptions::default()).unwrap();
        assert_eq!(
            request.body.unwrap(),
            RequestBodyBlock::FormData(HashMap::new())
        )
    }

    #[test]
    fn parse_body_form_data_invalid() {
        let text = r###"body type=form-data {
    key1 {
        type "unknown"
        value "value"
    }
}"###;
        assert!(parse(text, &ParseOptions::default()).is_err());
    }

    #[test]
    fn parse_body_form_data_single() {
        let text = r###"body type=form-data {
    key1 {
        type "text"
        value "value"
    }
}"###;
        let body1 = FormDataBodyItem {
            value: FormDataValue::Text("value".to_string()),
            ..Default::default()
        };
        let mut map = HashMap::new();
        map.insert("key1".to_string(), body1);
        let request = parse(text, &ParseOptions::default()).unwrap();
        assert_eq!(request.body.unwrap(), RequestBodyBlock::FormData(map))
    }

    #[test]
    fn parse_body_form_data_multiple() {
        let text = r###"body type=form-data {
    key1 {
        type "text"
        value "value"
    }
    key2 {
        type "file"
        value "path/to/file"
    }
}"###;
        let body1 = FormDataBodyItem {
            value: FormDataValue::Text("value".to_string()),
            ..Default::default()
        };
        let body2 = FormDataBodyItem {
            value: FormDataValue::File(PathBuf::from("path/to/file")),
            ..Default::default()
        };
        let mut map = HashMap::new();
        map.insert("key1".to_string(), body1);
        map.insert("key2".to_string(), body2);
        let request = parse(text, &ParseOptions::default()).unwrap();
        assert_eq!(request.body.unwrap(), RequestBodyBlock::FormData(map))
    }

    #[test]
    fn parse_body_urlencoded_empty() {
        let text = r###"body type=urlencoded {}"###;
        let request = parse(text, &ParseOptions::default()).unwrap();
        assert_eq!(
            request.body.unwrap(),
            RequestBodyBlock::UrlEncoded(HashMap::new())
        )
    }

    #[test]
    fn parse_body_urlencoded_single() {
        let text = r###"body type=urlencoded {
    key1 {
        value "value"
    }
}"###;
        let body1 = UrlEncodedBodyItem {
            value: "value".to_string(),
            ..Default::default()
        };
        let mut map = HashMap::new();
        map.insert("key1".to_string(), body1);
        let request = parse(text, &ParseOptions::default()).unwrap();
        assert_eq!(request.body.unwrap(), RequestBodyBlock::UrlEncoded(map))
    }

    #[test]
    fn parse_body_urlencoded_multiple() {
        let text = r###"body type=urlencoded {
    key1 {
        value "value"
    }
    key2 {
        value "value"
    }
}"###;
        let body1 = UrlEncodedBodyItem {
            value: "value".to_string(),
            ..Default::default()
        };
        let body2 = UrlEncodedBodyItem {
            value: "value".to_string(),
            ..Default::default()
        };
        let mut map = HashMap::new();
        map.insert("key1".to_string(), body1);
        map.insert("key2".to_string(), body2);
        let request = parse(text, &ParseOptions::default()).unwrap();
        assert_eq!(request.body.unwrap(), RequestBodyBlock::UrlEncoded(map))
    }

    #[test]
    fn parse_body_binary_missing_path() {
        let text = r###"body type=binary"###;
        assert!(parse(text, &ParseOptions::default()).is_err());
    }

    #[test]
    fn parse_body_binary() {
        let text = r###"body type=binary path="path/to/file""###;
        let request = parse(text, &ParseOptions::default()).unwrap();
        assert_eq!(
            request.body.unwrap(),
            RequestBodyBlock::Binary("path/to/file".into())
        )
    }
    #[test]
    #[ignore]
    fn manual_read_request_from_file_and_writing_back() {
        let content = fs::read_to_string(
            "tests/TestCollection/requests/MyFolder/Test6.request/Test6.get.sapic",
        )
        .unwrap();
        let request = super::parse(&content, &ParseOptions::default()).unwrap();
        println!("{:#?}", request.clone().body.unwrap());
        println!("{}", request.to_string());
    }
}
