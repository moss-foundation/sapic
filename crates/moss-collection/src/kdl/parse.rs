use anyhow::Result;
use kdl::{KdlDocument, KdlNode};

use crate::kdl::foundations::http::{HttpMethod, Metadata, Request, Url};
use crate::kdl::tokens::{METADATA_LIT, URL_LIT};

#[macro_export]
macro_rules! kdl_get_child {
    ( $kdl_node:expr, $child_name:expr ) => {
        $kdl_node.children().and_then(|c| c.get($child_name))
    };
}

#[macro_export]
macro_rules! kdl_get_arg_as_integer {
    ( $kdl_node:expr, $arg_name:expr) => {
        kdl_get_arg_as_helper!($kdl_node, $arg_name, as_integer)
    };
}

#[macro_export]
macro_rules! kdl_get_arg_as_str {
    ( $kdl_node:expr, $arg_name:expr) => {
        kdl_get_arg_as_helper!($kdl_node, $arg_name, as_string)
    };
}

#[macro_export]
macro_rules! kdl_get_arg_as_string {
    ( $kdl_node:expr, $arg_name:expr) => {
        kdl_get_arg_as_helper!($kdl_node, $arg_name, as_string).map(|value| value.to_string())
    };
}

macro_rules! kdl_get_arg_as_helper {
    ($kdl_node:expr, $arg_name:expr, $conversion:ident) => {
        $kdl_node
            .get_arg($arg_name)
            .and_then(|kdl_value| kdl_value.$conversion())
    };
}

fn parse_metadata_node(node: KdlNode) -> Result<Metadata> {
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

fn parse_url_node(node: KdlNode) -> Result<Url> {
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

pub fn parse(input: &str) -> Result<()> {
    let document: KdlDocument = input.parse()?;
    let mut request = Request::default();

    for node in document {
        match node.name().to_string().as_str() {
            METADATA_LIT => {
                request.metadata = Some(parse_metadata_node(node)?);
            }
            URL_LIT => {
                request.url = Some(parse_url_node(node)?);
            }
            _ => {}
        }
    }

    dbg!(request);

    Ok(())
}

#[cfg(test)]
mod tests {
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
