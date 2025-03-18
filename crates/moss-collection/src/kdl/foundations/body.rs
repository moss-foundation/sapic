use crate::{
    kdl::tokens::{BODY_LIT, RAW_STRING_INDENT, RAW_STRING_PREFIX, RAW_STRING_SUFFIX},
    models::types,
};
use kdl::{KdlDocument, KdlEntry, KdlIdentifier, KdlNode};

#[derive(Clone, Debug, PartialEq)]
pub enum RequestBody {
    // TODO: Raw(RawType), Binary, Form, File ...
    Raw(RawBodyType),
}

#[rustfmt::skip]
impl From<types::request_types::RequestBody> for RequestBody {
    fn from(body: types::request_types::RequestBody) -> Self {
        match body {
            types::request_types::RequestBody::Raw(raw_body) => RequestBody::Raw(RawBodyType::from(raw_body))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RawBodyType {
    Text(String),
    Json(String),
    Html(String),
    Xml(String),
}

#[rustfmt::skip]
impl From<types::request_types::RawBodyType> for RawBodyType {
    fn from(value: types::request_types::RawBodyType) -> Self {
        match value {
            types::request_types::RawBodyType::Text(text) => RawBodyType::Text(text),
            types::request_types::RawBodyType::Json(json) => RawBodyType::Json(json),
            types::request_types::RawBodyType::Html(html) => RawBodyType::Html(html),
            types::request_types::RawBodyType::Xml(xml) => RawBodyType::Xml(xml),
        }
    }
}

fn format_raw_string(content: &str) -> String {
    format!(
        "{RAW_STRING_PREFIX}\n\
        {content}\n\
        {RAW_STRING_SUFFIX}"
    )
    .lines()
    // Indent the content
    .map(|line| format!("{RAW_STRING_INDENT}{line}"))
    .collect::<Vec<_>>()
    .join("\n")
}

fn prepare_raw_body_node(node: &mut KdlNode, raw_body: RawBodyType) {
    let (typ, content) = match raw_body {
        RawBodyType::Text(s) => ("text", s),
        RawBodyType::Json(s) => ("json", s),
        RawBodyType::Html(s) => ("html", s),
        RawBodyType::Xml(s) => ("xml", s),
    };
    node.push(KdlEntry::new_prop("type", typ));
    let mut children = KdlDocument::new();
    // We have to manually format the output
    // Since the provided `autoformat` method will mess up with escape characters
    let formatted_content = format_raw_string(&content);
    let mut raw_content = KdlIdentifier::from(formatted_content.clone());
    // If we don't do this, the raw string quotes will be incorrectly escaped
    raw_content.set_repr(formatted_content);

    children.nodes_mut().push(KdlNode::new(raw_content));
    node.set_children(children);
}

impl Into<KdlNode> for RequestBody {
    fn into(self) -> KdlNode {
        let mut node = KdlNode::new(BODY_LIT);
        match self {
            RequestBody::Raw(raw_body) => {
                prepare_raw_body_node(&mut node, raw_body);
            }
        }
        node
    }
}
