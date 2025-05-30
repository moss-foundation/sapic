use uuid::Uuid;

pub struct SpecificationMetadata {
    pub id: Uuid,
}

// Items specs

pub enum RequestItemSpecificationModel {}

pub enum ItemSpecificationModelInner {
    Request(RequestItemSpecificationModel),
}

pub struct ItemSpecificationModel {
    pub metadata: SpecificationMetadata,
    pub inner: ItemSpecificationModelInner,
}

// Dirs specs

pub struct HttpDirSpecificationModel {}

pub enum RequestDirSpecificationModel {
    Http(HttpDirSpecificationModel),
}

pub enum DirSpecificationModelInner {
    Request(RequestDirSpecificationModel),
}

pub struct DirSpecificationModel {
    pub metadata: SpecificationMetadata,
    pub inner: DirSpecificationModelInner,
}

// impl DirSpecificationModel {
//     pub fn new<T>(metadata: SpecificationMetadata, ) -> Self {}
// }

// Specification model

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
