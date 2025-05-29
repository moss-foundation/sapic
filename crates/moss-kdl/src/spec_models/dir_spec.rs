pub mod component;
pub mod endpoint;
pub mod request;
pub mod schema;

use crate::spec_models::SpecificationMetadata;
use crate::spec_models::dir_spec::component::ComponentContent;
use crate::spec_models::dir_spec::endpoint::EndpointContent;
use crate::spec_models::dir_spec::request::RequestContent;
use crate::spec_models::dir_spec::schema::SchemaContent;
use kdl::KdlDocument;
use uuid::Uuid;

#[derive(Clone)]
pub enum DirContentByClass {
    Request(RequestContent),
    Endpoint(EndpointContent),
    Schema(SchemaContent),
    Component(ComponentContent),
}
#[derive(Clone)]
pub struct DirSpecificationModel {
    pub metadata: SpecificationMetadata,
    // On creation, we don't write anything beyond metadata
    pub content: Option<DirContentByClass>,
}

impl DirSpecificationModel {
    pub fn new(
        metadata: SpecificationMetadata,
        content: Option<DirContentByClass>,
    ) -> DirSpecificationModel {
        DirSpecificationModel { metadata, content }
    }

    pub fn id(&self) -> Uuid {
        self.metadata.id
    }
}

impl Into<KdlDocument> for DirSpecificationModel {
    fn into(self) -> KdlDocument {
        let mut doc = KdlDocument::new();
        doc.nodes_mut().push(self.metadata.clone().into());
        let content_doc: KdlDocument = self.content.clone().into();
        doc.nodes_mut().extend(content_doc.into_iter());
        doc
    }
}

impl<'a> Into<KdlDocument> for &'a DirSpecificationModel {
    fn into(self) -> KdlDocument {
        (*self).clone().into()
    }
}
