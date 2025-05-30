pub mod component;
pub mod endpoint;
pub mod request;
pub mod schema;

use crate::spec_models::SpecificationMetadata;
use crate::spec_models::item_spec::component::ComponentContent;
use crate::spec_models::item_spec::endpoint::EndpointContent;
use crate::spec_models::item_spec::request::RequestContent;
use crate::spec_models::item_spec::schema::SchemaContent;
use kdl::KdlDocument;
use uuid::Uuid;

#[derive(Clone)]
pub enum ItemContentByClass {
    Request(RequestContent),
    Endpoint(EndpointContent),
    Schema(SchemaContent),
    Component(ComponentContent),
}

impl Into<KdlDocument> for ItemContentByClass {
    fn into(self) -> KdlDocument {
        match self {
            ItemContentByClass::Request(content) => content.into(),
            ItemContentByClass::Endpoint(content) => content.into(),
            ItemContentByClass::Schema(content) => content.into(),
            ItemContentByClass::Component(content) => content.into(),
        }
    }
}

#[derive(Clone)]
pub struct ItemSpecificationModel {
    metadata: SpecificationMetadata,
    content: Option<ItemContentByClass>,
}

impl ItemSpecificationModel {
    pub fn new(metadata: SpecificationMetadata, content: Option<ItemContentByClass>) -> Self {
        Self { metadata, content }
    }

    pub fn id(&self) -> Uuid {
        self.metadata.id
    }
}

impl Into<KdlDocument> for ItemSpecificationModel {
    fn into(self) -> KdlDocument {
        let mut doc = KdlDocument::new();
        doc.nodes_mut().push(self.metadata.clone().into());
        let content_doc: KdlDocument = self.content.clone().into();
        doc.nodes_mut().extend(content_doc.into_iter());
        doc
    }
}

impl<'a> Into<KdlDocument> for &'a ItemSpecificationModel {
    fn into(self) -> KdlDocument {
        (*self).clone().into()
    }
}
