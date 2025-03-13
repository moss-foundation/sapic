use crate::kdl::foundations::body::RequestBody;
use crate::kdl::foundations::http::{
    HeaderParamBody, HeaderParamOptions, HttpRequestFile, PathParamBody, PathParamOptions,
    QueryParamBody, QueryParamOptions, Url,
};
use crate::kdl::tokens::*;
use anyhow::Result;
use kdl::{KdlDocument, KdlNode};
use std::collections::HashMap;
use thiserror::Error;
use crate::kdl::parse::ParseError::IllFormattedBody;
// FIXME: `KDLDocument::get_arg` assumes a node has only one argument
// So it cannot handle arrays such as `data 1 2 3 4 5`

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

fn parse_url_node(node: &KdlNode) -> Result<Url> {
    if let Some(document) = node.children() {
        Ok(Url {
            raw: kdl_get_arg_as_string!(document, "raw"),
            host: kdl_get_arg_as_string!(document, "host"),
        })
    } else {
        Ok(Url {
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

fn parse_body_node(node: &KdlNode) -> Result<RequestBody> {
    let typ = node
        .get("type")
        .ok_or_else(|| ParseError::MissingBodyType)
        .map(|value| value.as_string())?
        // If the type's value is not a string
        .ok_or_else(|| ParseError::InvalidBodyType)?;
    match typ {
        BODY_TYPE_TEXT => Ok(RequestBody::Text(parse_raw_body_content(&node)?)),
        BODY_TYPE_JAVASCRIPT => Ok(RequestBody::JavaScript(parse_raw_body_content(&node)?)),
        BODY_TYPE_JSON => Ok(RequestBody::Json(parse_raw_body_content(&node)?)),
        BODY_TYPE_HTML => Ok(RequestBody::HTML(parse_raw_body_content(&node)?)),
        BODY_TYPE_XML => Ok(RequestBody::XML(parse_raw_body_content(&node)?)),
        _ => Err(ParseError::InvalidBodyType.into()),
    }
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

    let result = raw_string.lines()
        .filter(|line| line.trim() != RAW_STRING_PREFIX && line.trim() != RAW_STRING_SUFFIX)
        .map(|line| line.strip_prefix(RAW_STRING_INDENT).ok_or(IllFormattedBody.into()))
        .collect::<Result<Vec<_>>>()?
        .join("\n");

    println!("{}", result);
    Ok(result)
}

pub fn parse(input: &str) -> Result<HttpRequestFile> {
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
                request.body = Some(parse_body_node(&node)?);
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
    use crate::kdl::foundations::body::RequestBody;
    use crate::kdl::foundations::http::{QueryParamBody, QueryParamOptions, Url};
    use kdl::{KdlDocument, KdlNode};
    use miette::{Diagnostic, NamedSource, SourceSpan};

    use std::fs;
    use thiserror::Error;
    // #[derive(Error, Debug, Diagnostic)]
    // #[error("oops!")]
    // #[diagnostic(
    //     code(oops::my::bad),
    //     url(docsrs),
    //     help("try doing it better next time?")
    // )]
    // struct MyBad {
    //     // The Source that we're gonna be printing snippets out of.
    //     // This can be a String if you don't have or care about file names.
    //     #[source_code]
    //     src: NamedSource<String>,
    //     // Snippets and highlights can be included in the diagnostic!
    //     #[label("This bit here")]
    //     bad_bit: SourceSpan,
    // }
    //
    // #[test]
    // fn miette() -> miette::Result<()> {
    //     // You can use plain strings as a `Source`, or anything that implements
    //     // the one-method `Source` trait.
    //     let src = "source\n  text\n    here".to_string();
    //
    //     Err(MyBad {
    //         src: NamedSource::new("bad_file.rs", src),
    //         bad_bit: (9, 4).into(),
    //     })?;
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn de() {
    //     let node =
    //         fs::read_to_string("./tests/requests/TestRequest/TestRequest.http.sapic").unwrap();
    //     // let doc: KdlDocument = content.parse().unwrap();
    //
    //     super::parse(&node).unwrap();
    // }

    #[test]
    fn test_parse_url_node_empty() {
        let text =
r#"url {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let url_node = parse_url_node(&node).unwrap();
        assert_eq!(
            url_node,
            Url {
                raw: None,
                host: None
            }
        );
        assert_eq!(url_node, Url::default())
    }

    #[test]
    fn test_parse_url_node_incomplete() {
        let text =
r#"url {
    raw "raw"
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let url_node = parse_url_node(&node).unwrap();
        assert_eq!(
            url_node,
            Url {
                raw: Some("raw".to_string()),
                host: None
            }
        );
    }

    #[test]
    fn test_parse_url_node_normal() {
        let text =
r#"url {
    raw "{{baseUrl}}/objects"
    host "{{baseUrl}}"
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let url_node = parse_url_node(&node).unwrap();
        assert_eq!(
            url_node,
            Url {
                raw: Some("{{baseUrl}}/objects".to_string()),
                host: Some("{{baseUrl}}".to_string())
            }
        )
    }

    #[test]
    fn test_parse_query_param_body_empty() {
        let text =
r#"param {
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
    fn test_parse_query_param_body_numeric_value() {
        let text =
r#"param {
    value 1
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_body = parse_query_param_body(&node).unwrap();
        assert_eq!(param_body.value, "1".to_string());
    }

    #[test]
    fn test_parse_query_param_body_full() {
        let text =
r#"param {
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
    fn test_parse_query_param_options_empty() {
        let text =
r#"options {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_query_param_options(&node).unwrap();
        assert_eq!(param_options, QueryParamOptions::default())
    }

    #[test]
    fn test_parse_query_param_options_normal() {
        let text =
r#"options {
    propagate #true
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_query_param_options(&node).unwrap();
        assert_eq!(param_options, QueryParamOptions { propagate: true })
    }

    #[test]
    fn test_parse_query_params_empty() {
        let text =
r#"params type=query {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let query_params = parse_query_params(&node).unwrap();
        assert!(query_params.is_empty());
    }

    #[test]
    fn test_parse_query_params_one_param() {
        let text =
r#"params type=query {
    pageToken {}
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let query_params = parse_query_params(&node).unwrap();
        assert_eq!(query_params.len(), 1);
        assert_eq!(query_params["pageToken"], QueryParamBody::default());
        let text =
r#"params type=query {
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
    fn test_parse_query_params_two_params() {
        let text =
r#"params type=query {
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
    fn test_parse_path_param_body_empty() {
        let text =
r#"param {
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
    fn test_parse_path_param_body_numeric_value() {
        let text =
r#"param {
    value 1
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_body = parse_path_param_body(&node).unwrap();
        assert_eq!(param_body.value, "1".to_string());
    }

    #[test]
    fn test_parse_path_param_body_full() {
        let text =
r#"param {
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
    fn test_parse_path_param_options_empty() {
        let text =
r#"options {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_path_param_options(&node).unwrap();
        assert_eq!(param_options, PathParamOptions::default())
    }

    #[test]
    fn test_parse_path_param_options_normal() {
        let text =
r#"options {
    propagate #true
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_path_param_options(&node).unwrap();
        assert_eq!(param_options, PathParamOptions { propagate: true })
    }

    #[test]
    fn test_parse_path_params_empty() {
        let text =
r#"params type=path {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let path_params = parse_path_params(&node).unwrap();
        assert!(path_params.is_empty());
    }

    #[test]
    fn test_parse_path_params_one_param() {
        let text =
r#"params type=path {
    pageToken {}
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let path_params = parse_path_params(&node).unwrap();
        assert_eq!(path_params.len(), 1);
        assert_eq!(path_params["pageToken"], PathParamBody::default());
        let text =
r#"params type=path {
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
    fn test_parse_path_params_two_params() {
        let text =
r#"params type=path {
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
    fn test_parse_header_body_empty() {
        let text =
r#"header {
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
    fn test_parse_header_body_numeric_value() {
        let text =
r#"header {
    value 1
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let header_body = parse_header_param_body(&node).unwrap();
        assert_eq!(header_body.value, "1".to_string());
    }

    #[test]
    fn test_parse_header_body_full() {
        let text =
r#"header {
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
    fn test_parse_header_param_options_empty() {
        let text =
r#"options {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_header_param_options(&node).unwrap();
        assert_eq!(param_options, HeaderParamOptions::default())
    }

    #[test]
    fn test_parse_header_param_options_normal() {
        let text =
r#"options {
    propagate #true
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let param_options = parse_header_param_options(&node).unwrap();
        assert_eq!(param_options, HeaderParamOptions { propagate: true })
    }

    #[test]
    fn test_parse_header_params_empty() {
        let text =
r#"headers {
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let header_params = parse_header_params(&node).unwrap();
        assert!(header_params.is_empty());
    }

    #[test]
    fn test_parse_header_params_one_param() {
        let text =
r#"headers {
    pageToken {}
}"#;
        let node = KdlNode::parse(&text).unwrap();
        let header_params = parse_header_params(&node).unwrap();
        assert_eq!(header_params.len(), 1);
        assert_eq!(header_params["pageToken"], HeaderParamBody::default());
        let text =
r#"headers {
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
    fn test_parse_header_params_two_params() {
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
    fn test_parse_body_json() {
        let text =
r###"body type=json {
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
        let json =
r#"{
    "key": "value",
    "object": {
        "inner": "value"
    }
}"#;
        let request = parse(text).unwrap();
        dbg!(&request.body.unwrap());
    }

    #[test]
    fn manual_test_read_request_from_file_and_writing_back() {
        let content = fs::read_to_string(
            "tests/TestCollection/requests/MyFolder/Test6.request/Test6.get.sapic",
        )
        .unwrap();
        let request = super::parse(&content).unwrap();
        println!("{:#?}", request.clone().body.unwrap());
        println!("{}", request.to_string());
    }


}
