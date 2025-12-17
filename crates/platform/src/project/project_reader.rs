use async_trait::async_trait;
use joinerror::ResultExt;
use moss_fs::FileSystem;
use sapic_base::project::manifest::{MANIFEST_FILE_NAME, ProjectManifest};
use sapic_core::context::AnyAsyncContext;
use sapic_system::project::ProjectReader;
use std::{path::Path, sync::Arc};

pub struct FsProjectReader {
    fs: Arc<dyn FileSystem>,
}

impl FsProjectReader {
    pub fn new(fs: Arc<dyn FileSystem>) -> Arc<Self> {
        Self { fs }.into()
    }
}

#[async_trait]
impl ProjectReader for FsProjectReader {
    async fn read_manifest(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
    ) -> joinerror::Result<ProjectManifest> {
        let manifest_path = abs_path.join(MANIFEST_FILE_NAME);

        let rdr = self
            .fs
            .open_file(ctx, &manifest_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to open manifest file: {}", manifest_path.display())
            })?;

        serde_json::from_reader(rdr).join_err_with::<()>(|| {
            format!("failed to parse manifest file: {}", manifest_path.display())
        })
    }
}
