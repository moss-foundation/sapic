use crate::kdl::tokens::{BODY_LIT, RAW_STRING_PREFIX, RAW_STRING_SUFFIX};
use kdl::{KdlDocument, KdlEntry, KdlIdentifier, KdlNode};

#[derive(Clone, Debug)]
pub enum RequestBody {
    Json(String),
}

impl Into<KdlNode> for RequestBody {
    fn into(self) -> KdlNode {
        let mut node = KdlNode::new(BODY_LIT);
        match self {
            RequestBody::Json(content) => {
                node.push(KdlEntry::new_prop("type", "json"));
                let mut children = KdlDocument::new();
                // We have to manually format the output
                // Since the provided `autoformat` method will mess up with escape characters
                let formatted_content = format!(
                    "    {RAW_STRING_PREFIX}\n{}\n    {RAW_STRING_SUFFIX}",
                    content
                );
                let mut json_content = KdlIdentifier::from(formatted_content.clone());

                // If we don't do this, the raw string quotes will be incorrectly escaped
                json_content.set_repr(formatted_content);

                children.nodes_mut().push(KdlNode::new(json_content));
                node.set_children(children);
            }
        }
        node
    }
}
