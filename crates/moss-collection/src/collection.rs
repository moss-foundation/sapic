use anyhow::{Context, Result};
use moss_file::toml;
use moss_fs::FileSystem;
use moss_storage::CollectionStorage;
use moss_storage::collection_storage::CollectionStorageImpl;
use std::{
    path::Path,
    sync::{Arc, atomic::AtomicUsize},
};

use tokio::sync::{OnceCell, RwLock};

use crate::config::{CONFIG_FILE_NAME, ConfigModel};
use crate::defaults;
use crate::manifest::{MANIFEST_FILE_NAME, ManifestModel, ManifestModelDiff};
use crate::worktree::Worktree;

pub struct Collection {
    fs: Arc<dyn FileSystem>,
    worktree: OnceCell<Arc<RwLock<Worktree>>>,
    abs_path: Arc<Path>,
    storage: Arc<dyn CollectionStorage>,
    next_entry_id: Arc<AtomicUsize>,
    manifest: toml::EditableFileHandle<ManifestModel>,
    config: toml::FileHandle<ConfigModel>,
}

pub struct CreateParams<'a> {
    pub name: Option<String>,
    pub internal_abs_path: &'a Path,
    pub external_abs_path: Option<&'a Path>,
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

        let storage = CollectionStorageImpl::new(&abs_path).context(format!(
            "Failed to open the collection {} state database",
            abs_path.display()
        ))?;

        let manifest =
            toml::EditableFileHandle::load(fs.clone(), abs_path.join(MANIFEST_FILE_NAME)).await?;

        let config = toml::FileHandle::load(fs.clone(), abs_path.join(CONFIG_FILE_NAME)).await?;

        Ok(Self {
            fs,
            abs_path: abs_path.to_owned().into(),
            worktree: OnceCell::new(),
            storage: Arc::new(storage),
            next_entry_id,
            manifest,
            config,
        })
    }

    pub async fn create<'a>(
        fs: Arc<dyn FileSystem>,
        next_entry_id: Arc<AtomicUsize>,
        params: CreateParams<'a>,
    ) -> Result<Self> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let storage = CollectionStorageImpl::new(&params.internal_abs_path).context(format!(
            "Failed to open the collection {} state database",
            params.internal_abs_path.display()
        ))?;

        let manifest_abs_path = if let Some(external_abs_path) = params.external_abs_path {
            external_abs_path.join(MANIFEST_FILE_NAME)
        } else {
            params.internal_abs_path.join(MANIFEST_FILE_NAME)
        };

        let manifest = toml::EditableFileHandle::create(
            fs.clone(),
            manifest_abs_path,
            ManifestModel {
                name: params
                    .name
                    .unwrap_or(defaults::DEFAULT_COLLECTION_NAME.to_string()),
            },
        )
        .await?;

        let config = toml::FileHandle::create(
            fs.clone(),
            params.internal_abs_path.join(CONFIG_FILE_NAME),
            ConfigModel {
                external_path: params.external_abs_path.map(|p| p.to_owned().into()),
            },
        )
        .await?;

        Ok(Self {
            fs: Arc::clone(&fs),
            abs_path: params.internal_abs_path.to_owned().into(),
            worktree: OnceCell::new(),
            storage: Arc::new(storage),
            next_entry_id,
            manifest,
            config,
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

    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    pub(super) fn storage(&self) -> &Arc<dyn CollectionStorage> {
        &self.storage
    }
}
