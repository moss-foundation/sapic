use crate::kdl::tokens::{BODY_LIT, RAW_STRING_INDENT, RAW_STRING_PREFIX, RAW_STRING_SUFFIX};
use kdl::{KdlDocument, KdlEntry, KdlIdentifier, KdlNode};

#[derive(Clone, Debug, PartialEq)]
pub enum RequestBody {
    Text(String),
    JavaScript(String),
    Json(String),
    HTML(String),
    XML(String),
}

fn format_raw_string(content: &str) -> String {
    format!(
        "{RAW_STRING_PREFIX}\n\
        {content}\n\
        {RAW_STRING_SUFFIX}"
    ).lines()
        // Indent the content
        .map(|line| format!("{RAW_STRING_INDENT}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn prepare_raw_body_node(node: &mut KdlNode, typ: &str, content: &str) {
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
            RequestBody::Json(content) => {
                prepare_raw_body_node(&mut node, "json", &content);
            }
            RequestBody::Text(content) => {
                prepare_raw_body_node(&mut node, "text", &content);
            }
            RequestBody::JavaScript(content) => {
                prepare_raw_body_node(&mut node, "javascript", &content);
            }
            RequestBody::HTML(content) => {
                prepare_raw_body_node(&mut node, "html", &content);
            }
            RequestBody::XML(content) => {
                prepare_raw_body_node(&mut node, "xml", &content);
            }
        }
        node
    }
}
