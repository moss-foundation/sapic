use kdl::KdlDocument;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SpecificationMetadata {
    pub id: Uuid,
}

impl From<KdlDocument> for SpecificationMetadata {
    fn from(document: KdlDocument) -> Self {
        todo!()
    }
}

// Items specs

pub enum RequestItemSpecificationModel {}

pub enum ItemSpecificationModelInner {
    Request(RequestItemSpecificationModel),
}

#[derive(Debug, Clone)]
pub struct ItemSpecificationModel {
    pub metadata: SpecificationMetadata,
    // pub inner: ItemSpecificationModelInner,
}

impl From<KdlDocument> for ItemSpecificationModel {
    fn from(document: KdlDocument) -> Self {
        todo!()
    }
}

// Dirs specs

pub struct HttpDirSpecificationModel {}

pub enum RequestDirSpecificationModel {
    Http(HttpDirSpecificationModel),
}

pub enum DirSpecificationModelInner {
    Request(RequestDirSpecificationModel),
}

#[derive(Debug, Clone)]
pub struct DirSpecificationModel {
    pub metadata: SpecificationMetadata,
    // pub inner: DirSpecificationModelInner,
}

impl From<KdlDocument> for DirSpecificationModel {
    fn from(document: KdlDocument) -> Self {
        todo!()
    }
}

// Specification model

#[derive(Debug, Clone)]
pub enum SpecificationModel {
    Item(ItemSpecificationModel),
    Dir(DirSpecificationModel),
}

impl SpecificationModel {
    pub fn id(&self) -> Uuid {
        match self {
            SpecificationModel::Item(item) => item.metadata.id,
            SpecificationModel::Dir(dir) => dir.metadata.id,
        }
    }
}

impl Into<KdlDocument> for SpecificationModel {
    fn into(self) -> KdlDocument {
        todo!()
    }
}
