use crate::adapters::kdl::foundations::http::{
    HttpMethod, Metadata, PathParamBody, PathParamOptions, QueryParamBody, QueryParamOptions,
    Request, Url,
};
use crate::adapters::kdl::tokens::*;
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

fn parse_metadata_node(node: &KdlNode) -> Result<Metadata> {
    if let Some(document) = node.children() {
        Ok(Metadata {
            order: kdl_get_arg_as_integer!(document, "order")
                .and_then(|value| Some(value as usize)),
            method: kdl_get_arg_as_str!(document, "method")
                .and_then(|value| match value {
                    "GET" => Some(HttpMethod::Get),
                    "POST" => Some(HttpMethod::Post),
                    _ => Some(HttpMethod::default()),
                })
                .unwrap_or_default(),
        })
    } else {
        Ok(Metadata {
            order: None,
            method: HttpMethod::default(),
        })
    }
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
        let value = kdl_get_arg_as_value!(fields, "value");
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
            value,
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
        let value = kdl_get_arg_as_value!(fields, "value");
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
            value,
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

pub fn parse(input: &str) -> Result<()> {
    let document: KdlDocument = input.parse()?;
    let mut request = Request::default();

    for node in document {
        match node.name().to_string().as_str() {
            METADATA_LIT => {
                request.metadata = Some(parse_metadata_node(&node)?);
            }
            URL_LIT => {
                request.url = Some(parse_url_node(&node)?);
            }
            PARAMS_LIT => {
                // FIXME: Should we handle duplicate query/path param nodes?
                let typ = node
                    .get("type")
                    .and_then(|value| value.as_string())
                    .ok_or_else(|| ParseError::InvalidParamsType)?;

                match typ {
                    QUERY_LIT => {
                        request.query_params = Some(parse_query_params(&node)?);
                    }
                    PATH_LIT => {
                        request.path_params = Some(parse_path_params(&node)?);
                    }
                    _ => return Err(ParseError::InvalidParamsType.into()),
                }
            }
            _ => {}
        }
    }

    dbg!(request);

    Ok(())
}

#[cfg(test)]
mod tests {
    use kdl::KdlDocument;
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
}
