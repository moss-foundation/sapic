pub mod resource_edit_service;
pub mod resource_service;

use async_trait::async_trait;
use json_patch::PatchOperation;
use moss_edit::json::EditOptions;
use moss_fs::utils::SanitizedPath;
use sapic_base::resource::types::primitives::ResourceKind;
use sapic_core::context::AnyAsyncContext;
use std::{path::Path, sync::Arc};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct ScannedEntry {
    // pub id: ResourceId,
    pub name: String,
    pub path: Arc<Path>,
    // pub class: ResourceClass,
    pub kind: ResourceKind,
    // pub protocol: Option<ResourceProtocol>,
}

#[async_trait]
pub trait ResourceBackend: Send + Sync {
    async fn scan(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        sender: mpsc::UnboundedSender<ScannedEntry>,
    ) -> joinerror::Result<()>;

    async fn remove_entry(&self, ctx: &dyn AnyAsyncContext, path: &Path) -> joinerror::Result<()>;
    async fn create_entry(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: SanitizedPath,
        content: &[u8],
        is_dir: bool,
    ) -> joinerror::Result<()>;
}

pub struct ResourceRenameParams<'a> {
    pub abs_path: &'a Path,
    pub from: &'a Path,
    pub to: &'a Path,
}

pub struct ResourceEditParams<'a> {
    pub name: Option<ResourceRenameParams<'a>>,
    pub patches: &'a [(PatchOperation, EditOptions)],
}

#[async_trait]
pub trait ResourceEditBackend: Send + Sync {
    async fn edit<'a>(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ResourceEditParams<'a>,
    ) -> joinerror::Result<()>;
}
