use anyhow::{Context, Result};
use moss_environment::environment::Environment;
use moss_file::toml::{self, TomlFileHandle};
use moss_fs::FileSystem;
use moss_storage::{CollectionStorage, collection_storage::CollectionStorageImpl};
use std::{collections::HashMap, path::Path, sync::Arc};
use uuid::Uuid;

use tokio::sync::OnceCell;

use crate::{
    config::{CONFIG_FILE_NAME, ConfigModel},
    defaults, dirs,
    manifest::{MANIFEST_FILE_NAME, ManifestModel, ManifestModelDiff},
    worktree::Worktree,
};

pub struct EnvironmentItem {
    pub id: Uuid,
    pub name: String,
    pub inner: Environment,
}

type EnvironmentMap = HashMap<Uuid, Arc<EnvironmentItem>>;

pub struct Collection {
    #[allow(dead_code)]
    fs: Arc<dyn FileSystem>,
    worktree: Arc<Worktree>,
    abs_path: Arc<Path>,
    #[allow(dead_code)]
    storage: Arc<dyn CollectionStorage>,
    #[allow(dead_code)]
    environments: OnceCell<EnvironmentMap>,
    manifest: toml::EditableInPlaceFileHandle<ManifestModel>,
    #[allow(dead_code)]
    config: TomlFileHandle<ConfigModel>,
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
    pub async fn load(abs_path: &Path, fs: Arc<dyn FileSystem>) -> Result<Self> {
        let abs_path: Arc<Path> = abs_path.to_owned().into();
        debug_assert!(abs_path.is_absolute());

        let storage = CollectionStorageImpl::new(&abs_path).context(format!(
            "Failed to open the collection {} state database",
            abs_path.display()
        ))?;

        let manifest =
            toml::EditableInPlaceFileHandle::load(fs.clone(), abs_path.join(MANIFEST_FILE_NAME))
                .await?;

        let config = TomlFileHandle::load(fs.clone(), &abs_path.join(CONFIG_FILE_NAME)).await?;
        let worktree = Worktree::new(fs.clone(), abs_path.clone());

        // TODO: Load environments

        Ok(Self {
            fs,
            abs_path,
            worktree: Arc::new(worktree),
            storage: Arc::new(storage),
            environments: OnceCell::new(),
            manifest,
            config,
        })
    }

    pub async fn create<'a>(fs: Arc<dyn FileSystem>, params: CreateParams<'a>) -> Result<Self> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let storage = CollectionStorageImpl::new(&params.internal_abs_path).context(format!(
            "Failed to open the collection {} state database",
            params.internal_abs_path.display()
        ))?;

        let abs_path: Arc<Path> = params
            .external_abs_path
            .unwrap_or(params.internal_abs_path)
            .to_owned()
            .into();

        for dir in &[
            dirs::REQUESTS_DIR,
            dirs::ENDPOINTS_DIR,
            dirs::COMPONENTS_DIR,
            dirs::SCHEMAS_DIR,
            dirs::ENVIRONMENTS_DIR,
        ] {
            fs.create_dir(&abs_path.join(dir)).await?;
        }

        let worktree = Worktree::new(fs.clone(), abs_path.clone());
        let manifest = toml::EditableInPlaceFileHandle::create(
            fs.clone(),
            abs_path.join(MANIFEST_FILE_NAME),
            ManifestModel {
                name: params
                    .name
                    .unwrap_or(defaults::DEFAULT_COLLECTION_NAME.to_string()),
            },
        )
        .await?;

        let config = TomlFileHandle::create(
            fs.clone(),
            &params.internal_abs_path.join(CONFIG_FILE_NAME),
            ConfigModel {
                external_path: params.external_abs_path.map(|p| p.to_owned().into()),
            },
        )
        .await?;

        // TODO: Load environments

        Ok(Self {
            fs: Arc::clone(&fs),
            abs_path: params.internal_abs_path.to_owned().into(),
            worktree: Arc::new(worktree),
            storage: Arc::new(storage),
            environments: OnceCell::new(),
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

    pub fn worktree(&self) -> Arc<Worktree> {
        self.worktree.clone()
    }

    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    #[allow(dead_code)]
    pub(super) fn storage(&self) -> &Arc<dyn CollectionStorage> {
        &self.storage
    }

    pub async fn environments(&self) -> Result<&EnvironmentMap> {
        let result = self
            .environments
            .get_or_try_init(|| async move {
                let environments = HashMap::new();
                Ok::<_, anyhow::Error>(environments)
            })
            .await?;

        Ok(result)
    }
}
