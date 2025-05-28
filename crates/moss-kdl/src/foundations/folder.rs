use anyhow::Result;
use kdl::KdlDocument;

#[derive(Clone, Debug, Default)]
pub struct FolderSpecification {
    // TODO: Folder specific content
}

impl FolderSpecification {
    pub fn new() -> Self {
        Self {}
    }
    pub fn parse(document: KdlDocument) -> Result<Self> {
        Ok(FolderSpecification {})
    }
}

impl Into<KdlDocument> for FolderSpecification {
    fn into(self) -> KdlDocument {
        KdlDocument::new()
    }
}
