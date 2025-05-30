use kdl::KdlDocument;

#[derive(Clone)]
pub enum EndpointContent {}

impl Into<KdlDocument> for EndpointContent {
    fn into(self) -> KdlDocument {
        todo!()
    }
}
