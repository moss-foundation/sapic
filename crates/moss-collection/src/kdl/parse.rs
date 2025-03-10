use crate::kdl::foundations::body::RequestBody;
use crate::kdl::foundations::http::{
    HeaderOptions, HeaderParamBody, HttpRequestFile, PathParamBody, PathParamOptions,
    QueryParamBody, QueryParamOptions, Url,
};
use crate::kdl::tokens::*;
use anyhow::Result;
use kdl::{KdlDocument, KdlNode};
use std::collections::HashMap;
use thiserror::Error;
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
macro_rules! kdl_get_arg_as_str {
    ( $kdl_document:expr, $arg_name:expr) => {
        kdl_get_arg_as_helper!($kdl_document, $arg_name, as_string)
    };
}

#[macro_export]
macro_rules! kdl_get_arg_as_string {
    ( $kdl_document:expr, $arg_name:expr) => {
        kdl_get_arg_as_helper!($kdl_document, $arg_name, as_string).map(|value| value.to_string())
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

fn parse_headers_node(node: &KdlNode) -> Result<HashMap<String, HeaderParamBody>> {
    let mut headers: HashMap<String, HeaderParamBody> = HashMap::new();
    if let Some(document) = node.children() {
        for header_node in document.nodes() {
            let name = header_node.name().to_string();
            let header_body = parse_header_body(header_node)?;
            headers.insert(name, header_body);
        }
    }
    Ok(headers)
}

fn parse_header_body(node: &KdlNode) -> Result<HeaderParamBody> {
    if let Some(fields) = node.children() {
        let value = kdl_get_arg_as_string!(fields, "value");
        let desc = kdl_get_arg_as_string!(fields, "desc");
        let order = kdl_get_arg_as_integer!(fields, "order").and_then(|value| Some(value as usize));
        let disabled = kdl_get_arg_as_bool!(fields, "disabled").unwrap_or(false);
        let options_node = fields.get("options");
        let options = if let Some(options_node) = options_node {
            parse_header_options(options_node)?
        } else {
            HeaderOptions::default()
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

fn parse_header_options(node: &KdlNode) -> Result<HeaderOptions> {
    if let Some(fields) = node.children() {
        let propagate = kdl_get_arg_as_bool!(fields, "propagate").unwrap_or(false);
        Ok(HeaderOptions { propagate })
    } else {
        Ok(HeaderOptions::default())
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
        BODY_TYPE_JSON => parse_json_body(node),
        _ => Err(ParseError::InvalidBodyType.into()),
    }
}

fn parse_json_body(node: &KdlNode) -> Result<RequestBody> {
    let raw_content = node
        .children()
        .ok_or(ParseError::EmptyBody)?
        .nodes()
        .into_iter()
        .next()
        .ok_or(ParseError::EmptyBody)?
        .name()
        .to_string();

    let json = raw_content
        .strip_prefix(RAW_STRING_PREFIX)
        .ok_or(ParseError::IllFormattedBody)?
        .strip_suffix(RAW_STRING_SUFFIX)
        .ok_or(ParseError::IllFormattedBody)?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(RequestBody::Json(json.to_string()))
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
                request.headers = parse_headers_node(&node)?;
            }

            _ => {}
        }
    }

    Ok(request)
}

#[cfg(test)]
mod tests {
    use crate::kdl::foundations::body::RequestBody;
    use crate::kdl::foundations::http::{QueryParamBody, QueryParamOptions, Url};

    use kdl::{KdlDocument, KdlNode};
    use miette::{Diagnostic, NamedSource, SourceSpan};
    use std::fs;
    use thiserror::Error;

    #[derive(Error, Debug, Diagnostic)]
    #[error("oops!")]
    #[diagnostic(
        code(oops::my::bad),
        url(docsrs),
        help("try doing it better next time?")
    )]
    struct MyBad {
        // The Source that we're gonna be printing snippets out of.
        // This can be a String if you don't have or care about file names.
        #[source_code]
        src: NamedSource<String>,
        // Snippets and highlights can be included in the diagnostic!
        #[label("This bit here")]
        bad_bit: SourceSpan,
    }

    #[test]
    fn miette() -> miette::Result<()> {
        // You can use plain strings as a `Source`, or anything that implements
        // the one-method `Source` trait.
        let src = "source\n  text\n    here".to_string();

        Err(MyBad {
            src: NamedSource::new("bad_file.rs", src),
            bad_bit: (9, 4).into(),
        })?;

        Ok(())
    }

    #[test]
    fn de() {
        let node =
            fs::read_to_string("./tests/requests/TestRequest/TestRequest.http.sapic").unwrap();
        // let doc: KdlDocument = content.parse().unwrap();

        super::parse(&node).unwrap();
    }

    #[test]
    fn test_url_to_string() {
        let url = Url {
            raw: Some("raw".to_string()),
            host: Some("host".to_string()),
        };
        let mut node: KdlNode = url.into();
        node.autoformat();
        println!("{}", node.to_string());
    }

    #[test]
    fn test_query_param_body_to_string() {
        let body = QueryParamBody {
            value: "value".into(),
            desc: Some("desc".into()),
            order: Some(1),
            disabled: false,
            options: QueryParamOptions { propagate: true },
        };
        let mut doc: KdlDocument = body.into();
        doc.autoformat();
        println!("{}", doc.to_string());
    }

    #[test]
    fn test_read_request_from_file_and_writing_back() {
        let content = fs::read_to_string(
            "tests/TestCollection/requests/MyFolder/Test6.request/Test6.get.sapic",
        )
        .unwrap();
        let request = super::parse(&content).unwrap();
        println!("{}", request.to_string());
    }

    #[test]
    fn test_body() {
        let json = "{\n    \"key\": \"value\"\n}";

        let body = RequestBody::Json(json.to_string());
        let node: KdlNode = body.into();
        fs::write("test_output.kdl", node.to_string()).unwrap();
    }

    #[test]
    fn test_raw_string() {
        let document = KdlDocument::parse(&fs::read_to_string("test.kdl").unwrap()).unwrap();
        let body_node = document.nodes().into_iter().next().unwrap();
        let inner = body_node.children().unwrap();
        dbg!(inner.nodes().into_iter().next().unwrap());
    }
}
