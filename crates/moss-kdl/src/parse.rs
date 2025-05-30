pub mod http;

use thiserror::Error;

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

pub struct HttpRequestParseOptions {
    html_parse_mode: HtmlParseMode,
}

impl Default for HttpRequestParseOptions {
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

#[macro_export]
macro_rules! kdl_get_arg_as_helper {
    ($kdl_document:expr, $arg_name:expr, $conversion:ident) => {
        $kdl_document
            .get_arg($arg_name)
            .and_then(|kdl_value| kdl_value.$conversion())
    };
}
