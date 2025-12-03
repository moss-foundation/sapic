use async_trait::async_trait;
use sapic_base::project::manifest::ProjectManifest;
use std::path::Path;

// pub mod project_edit_service;
// pub mod project_service;

#[async_trait]
pub trait ProjectReader: Send + Sync {
    async fn read_manifest(&self, abs_path: &Path) -> joinerror::Result<ProjectManifest>;
}

#[async_trait]

pub trait ProjectServiceFs: Send + Sync {}
