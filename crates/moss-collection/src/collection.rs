use anyhow::{Context, Result};
use moss_file::toml::EditableFileHandle;
use moss_fs::FileSystem;
use moss_storage::CollectionStorage;
use moss_storage::collection_storage::CollectionStorageImpl;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::{path::PathBuf, sync::Arc};

use tokio::sync::{OnceCell, RwLock};

use crate::defaults;
use crate::manifest::{MANIFEST_FILE_NAME, ManifestModel, ManifestModelDiff};
use crate::worktree::Worktree;

pub struct Collection {
    fs: Arc<dyn FileSystem>,
    worktree: OnceCell<Arc<RwLock<Worktree>>>,
    abs_path: PathBuf,
    storage: Arc<dyn CollectionStorage>,
    next_entry_id: Arc<AtomicUsize>,
    manifest: EditableFileHandle<ManifestModel>,
}

pub struct CreateParams {
    pub name: Option<String>,
}

pub struct ModifyParams {
    pub name: Option<String>,
}

impl Collection {
    pub async fn load(
        abs_path: &Path,
        fs: Arc<dyn FileSystem>,
        next_entry_id: Arc<AtomicUsize>,
    ) -> Result<Self> {
        debug_assert!(abs_path.is_absolute());

        let manifest = EditableFileHandle::load(fs.clone(), abs_path.join(MANIFEST_FILE_NAME)).await?;

        let storage = CollectionStorageImpl::new(&abs_path).context(format!(
            "Failed to open the collection {} state database",
            abs_path.display()
        ))?;

        Ok(Self {
            fs,
            abs_path: abs_path.to_owned().into(),
            worktree: OnceCell::new(),
            storage: Arc::new(storage),
            next_entry_id,
            manifest,
        })
    }

    pub async fn create(
        abs_path: &Path,
        fs: Arc<dyn FileSystem>,
        next_entry_id: Arc<AtomicUsize>,
        params: CreateParams,
    ) -> Result<Self> {
        debug_assert!(abs_path.is_absolute());

        let storage = CollectionStorageImpl::new(&abs_path).context(format!(
            "Failed to open the collection {} state database",
            abs_path.display()
        ))?;

        let manifest = EditableFileHandle::new(
            fs.clone(),
            abs_path.join(MANIFEST_FILE_NAME),
            ManifestModel {
                name: params
                    .name
                    .unwrap_or(defaults::DEFAULT_COLLECTION_NAME.to_string()),
            },
        )
        .await?;

        Ok(Self {
            fs: Arc::clone(&fs),
            abs_path: abs_path.to_owned().into(),
            worktree: OnceCell::new(),
            storage: Arc::new(storage),
            next_entry_id,
            manifest,
        })
    }

    pub async fn modify(&self, params: ModifyParams) -> Result<()> {
        if params.name.is_some() {
            self.manifest
                .edit(ManifestModelDiff {
                    name: params.name.to_owned(),
                })
                .await?;
        }

        Ok(())
    }

    pub async fn manifest(&self) -> ManifestModel {
        self.manifest.model().await
    }

    pub async fn worktree(&self) -> Result<&Arc<RwLock<Worktree>>> {
        self.worktree
            .get_or_try_init(|| async move {
                let worktree = Worktree::new(
                    self.fs.clone(),
                    Arc::from(self.abs_path.clone()),
                    self.next_entry_id.clone(),
                );

                Ok(Arc::new(RwLock::new(worktree)))
            })
            .await
    }

    pub fn abs_path(&self) -> &PathBuf {
        &self.abs_path
    }

    pub(super) fn storage(&self) -> &Arc<dyn CollectionStorage> {
        &self.storage
    }
}
