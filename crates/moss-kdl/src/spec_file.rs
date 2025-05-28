use crate::foundations::folder::FolderSpecification;
use crate::foundations::http::HttpRequestFile;
use crate::parse::{HttpRequestParseOptions, http};
use crate::tokens::METADATA_LIT;
use crate::{kdl_get_arg_as_string, kdl_get_arg_as_value};
use anyhow::{Result, anyhow};
use kdl::{KdlDocument, KdlEntry, KdlNode};
use uuid::Uuid;

pub enum SpecificationFileType {
    Http,
    Folder,
}

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
    fn try_from(node: KdlNode) -> Result<Self, Self::Error> {
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

#[derive(Clone)]
pub struct SpecificationFile<T>
where
    T: Clone + Into<KdlDocument>,
{
    model: T,
    metadata: SpecificationMetadata,
}

impl<T> Into<KdlDocument> for SpecificationFile<T>
where
    T: Clone + Into<KdlDocument>,
{
    fn into(self) -> KdlDocument {
        let mut doc = KdlDocument::new();
        doc.nodes_mut().push(self.metadata.clone().into());
        let model_doc: KdlDocument = self.model.clone().into();
        doc.nodes_mut().extend(model_doc.into_iter());
        doc
    }
}

impl<T> SpecificationFile<T>
where
    T: Clone + Into<KdlDocument>,
{
    pub fn new(model: T, metadata: SpecificationMetadata) -> Self {
        Self { model, metadata }
    }
}

fn parse_metadata(document: &KdlDocument) -> Result<SpecificationMetadata> {
    let metadata_node = document
        .get(METADATA_LIT)
        .ok_or(anyhow!("Metadata node missing"))?
        .to_owned();
    let id = if let Some(fields) = metadata_node.children() {
        let id_str = kdl_get_arg_as_string!(fields, "id")
            .ok_or(anyhow!("Cannot get 'id' field from the metadata node"))?;
        Uuid::parse_str(&id_str).map_err(|_| anyhow!("Invalid UUID: {}", id_str))?
    } else {
        return Err(anyhow!("Metadata node has no children"));
    };
    Ok(SpecificationMetadata { id })
}

pub fn parse_http_request_file(
    text: &str,
    opts: HttpRequestParseOptions,
) -> Result<SpecificationFile<HttpRequestFile>> {
    let document = KdlDocument::parse(text)?;
    let metadata = parse_metadata(&document)?;
    let model = HttpRequestFile::parse(document, &opts)?;
    Ok(SpecificationFile::new(model, metadata))
}

pub fn parse_folder_specfile(text: &str) -> Result<SpecificationFile<FolderSpecification>> {
    let document = KdlDocument::parse(text)?;
    let metadata = parse_metadata(&document)?;
    let model = FolderSpecification::parse(document)?;
    Ok(SpecificationFile::new(model, metadata))
}
