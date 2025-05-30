pub mod dir_spec;
pub mod entry_spec;
pub mod item_spec;

use anyhow::anyhow;
use kdl::{KdlDocument, KdlEntry, KdlNode};
use uuid::Uuid;

use crate::tokens::METADATA_LIT;

#[derive(Clone)]
pub struct SpecificationMetadata {
    pub id: Uuid,
}

impl Into<KdlNode> for SpecificationMetadata {
    fn into(self) -> KdlNode {
        let mut node = KdlNode::new(METADATA_LIT);
        let mut children = KdlDocument::new();
        let mut id_node = KdlNode::new("id");
        id_node.push(KdlEntry::new(self.id.to_string()));
        children.nodes_mut().push(id_node);
        node.set_children(children);
        node
    }
}

impl TryFrom<KdlNode> for SpecificationMetadata {
    // TODO: proper error handling
    type Error = anyhow::Error;
    fn try_from(node: KdlNode) -> anyhow::Result<Self, Self::Error> {
        if let Some(fields) = node.children() {
            let id_str = fields
                .get_arg("id")
                .ok_or_else(|| anyhow!("Missing 'id' field from the metadata node"))?
                .to_string();
            let id = Uuid::parse_str(&id_str);
            if let Ok(id) = id {
                Ok(SpecificationMetadata { id })
            } else {
                Err(anyhow!("Invalid uuid: {}", id_str))
            }
        } else {
            Err(anyhow!("Metadata node has no children"))
        }
    }
}
