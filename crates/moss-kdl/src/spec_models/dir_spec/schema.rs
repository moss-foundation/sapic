use kdl::KdlDocument;

#[derive(Clone)]
pub enum SchemaContent {}

impl Into<KdlDocument> for SchemaContent {
    fn into(self) -> KdlDocument {
        todo!()
    }
}
