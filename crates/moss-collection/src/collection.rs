use anyhow::{Context, Result};
use moss_file::toml::EditableToml;
use moss_fs::{FileSystem, RenameOptions};
use moss_storage::CollectionStorage;
use moss_storage::collection_storage::CollectionStorageImpl;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::{path::PathBuf, sync::Arc};
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock};

use crate::defaults;
use crate::manifest::{MANIFEST_FILE_NAME, ManifestModel, ManifestModelDiff};
use crate::worktree::Worktree;

#[derive(Debug, Error)]
pub enum CollectionError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type CollectionResult<T> = Result<T, CollectionError>;

pub struct Collection {
    fs: Arc<dyn FileSystem>,
    worktree: OnceCell<Arc<RwLock<Worktree>>>,
    abs_path: PathBuf,
    pub(crate) storage: Arc<dyn CollectionStorage>,
    next_entry_id: Arc<AtomicUsize>,
    manifest: EditableToml<ManifestModel>,
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
    ) -> CollectionResult<Self> {
        debug_assert!(abs_path.is_absolute());

        if !abs_path.exists() {
            return Err(CollectionError::NotFound(abs_path.display().to_string()));
        }

        let manifest = EditableToml::load(fs.clone(), abs_path.join(MANIFEST_FILE_NAME))
            .await
            .map_err(CollectionError::Other)?;

        let storage = match CollectionStorageImpl::new(&abs_path).context(format!(
            "Failed to open the collection {} state database",
            abs_path.display()
        )) {
            Ok(storage) => storage,
            Err(e) => {
                return Err(CollectionError::Internal(e.to_string()));
            }
        };

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
    ) -> CollectionResult<Self> {
        debug_assert!(abs_path.is_absolute());

        let storage = match CollectionStorageImpl::new(&abs_path).context(format!(
            "Failed to open the collection {} state database",
            abs_path.display()
        )) {
            Ok(storage) => storage,
            Err(e) => {
                return Err(CollectionError::Internal(e.to_string()));
            }
        };

        let manifest = EditableToml::new(
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

    pub async fn modify(&self, params: ModifyParams) -> CollectionResult<()> {
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

    pub async fn reset(&mut self, new_path: Arc<Path>) -> Result<()> {
        debug_assert!(new_path.is_absolute());

        let old_path = std::mem::replace(&mut self.abs_path, new_path.to_path_buf());
        let fs_clone = self.fs.clone();
        let new_path_clone = new_path.clone();

        let after_drop = Box::pin(async move {
            fs_clone
                .rename(&old_path, &new_path_clone, RenameOptions::default())
                .await?;

            Ok(())
        });

        self.storage.reset(&new_path, after_drop).await?;

        Ok(())
    }
}
