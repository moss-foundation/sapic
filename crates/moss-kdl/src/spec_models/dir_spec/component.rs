use kdl::KdlDocument;

#[derive(Clone)]
pub enum ComponentContent {}

impl Into<KdlDocument> for ComponentContent {
    fn into(self) -> KdlDocument {
        todo!()
    }
}
