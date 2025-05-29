use kdl::KdlDocument;

#[derive(Clone)]
pub enum RequestContent {
    // TODO: Request dir specification content
}

impl Into<KdlDocument> for RequestContent {
    fn into(self) -> KdlDocument {
        KdlDocument::new()
    }
}
