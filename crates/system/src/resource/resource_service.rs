use moss_fs::utils::SanitizedPath;
use moss_text::sanitized::sanitize;
use sapic_core::context::AnyAsyncContext;
use std::{path::Path, sync::Arc};
use tokio::sync::mpsc;

use crate::resource::{ResourceBackend, ScannedEntry};

pub struct ResourceService {
    backend: Arc<dyn ResourceBackend>,
}

impl ResourceService {
    pub fn new(backend: Arc<dyn ResourceBackend>) -> Self {
        Self { backend }
    }

    pub async fn resources(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        sender: mpsc::UnboundedSender<ScannedEntry>,
    ) -> joinerror::Result<()> {
        self.backend.scan(ctx, path, sender).await
    }

    pub async fn remove_entry(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
    ) -> joinerror::Result<()> {
        self.backend.remove_entry(ctx, path).await
    }

    pub async fn create_entry(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        name: &str,
        content: &[u8],
        is_dir: bool,
    ) -> joinerror::Result<()> {
        let sanitized_path: SanitizedPath = moss_fs::utils::sanitize_path(path, None)?
            .join(sanitize(name))
            .into();

        self.backend
            .create_entry(ctx, sanitized_path, content, is_dir)
            .await
    }
}
