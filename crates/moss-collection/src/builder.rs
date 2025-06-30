use anyhow::{Context, Result};
use moss_applib::{ServiceMarker, providers::ServiceMap, subscription::EventEmitter};
use moss_file::toml::TomlFileHandle;
use moss_fs::FileSystem;
use moss_git::url::normalize_git_url;
use moss_hcl::Block;
use moss_storage::collection_storage::CollectionStorageImpl;
use std::{
    any::TypeId,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;

use crate::{
    Collection,
    config::{CONFIG_FILE_NAME, ConfigModel},
    constants::COLLECTION_ICON_FILENAME,
    defaults,
    dirs::{self, ASSETS_DIR},
    manifest::{MANIFEST_FILE_NAME, ManifestModel},
    models::types::configuration::docschema::{
        RawDirComponentConfiguration, RawDirConfiguration, RawDirEndpointConfiguration,
        RawDirRequestConfiguration, RawDirSchemaConfiguration,
    },
    services::set_icon::{SetIconService, constants::ICON_SIZE},
    worktree::Worktree,
};

const OTHER_DIRS: [&str; 2] = [dirs::ASSETS_DIR, dirs::ENVIRONMENTS_DIR];

const WORKTREE_DIRS: [&str; 4] = [
    dirs::REQUESTS_DIR,
    dirs::ENDPOINTS_DIR,
    dirs::COMPONENTS_DIR,
    dirs::SCHEMAS_DIR,
];

pub struct CreateParams<'a> {
    pub name: Option<String>,
    pub internal_abs_path: &'a Path,
    pub external_abs_path: Option<&'a Path>,
    pub repository: Option<String>,
    pub icon_path: Option<PathBuf>,
}

pub struct LoadParams<'a> {
    pub internal_abs_path: &'a Path,
}

pub struct CollectionBuilder {
    fs: Arc<dyn FileSystem>,
    services: ServiceMap,
}

impl CollectionBuilder {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self {
            fs,
            services: Default::default(),
        }
    }

    pub fn with_service<T: ServiceMarker + Send + Sync>(mut self, service: T) -> Self {
        self.services.insert(TypeId::of::<T>(), Arc::new(service));
        self
    }

    pub async fn load<'a>(self, params: LoadParams<'a>) -> Result<Collection> {
        let abs_path: Arc<Path> = params.internal_abs_path.to_owned().into();
        debug_assert!(abs_path.is_absolute());

        let storage = CollectionStorageImpl::new(&abs_path).context(format!(
            "Failed to open the collection {} state database",
            abs_path.display()
        ))?;

        let manifest = moss_file::toml::EditableInPlaceFileHandle::load(
            self.fs.clone(),
            abs_path.join(MANIFEST_FILE_NAME),
        )
        .await?;

        let config =
            TomlFileHandle::load(self.fs.clone(), &abs_path.join(CONFIG_FILE_NAME)).await?;
        let worktree = Worktree::new(self.fs.clone(), abs_path.clone());

        // TODO: Load environments

        Ok(Collection {
            fs: self.fs.clone(),
            services: self.services.into(),
            abs_path: abs_path.clone(),
            worktree: Arc::new(worktree),
            storage: Arc::new(storage),
            environments: OnceCell::new(),
            manifest,
            config,
            on_did_change: EventEmitter::new(),
        })
    }

    pub async fn create<'a>(self, params: CreateParams<'a>) -> Result<Collection> {
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

        let worktree = Worktree::new(self.fs.clone(), abs_path.clone());
        for dir in &WORKTREE_DIRS {
            let content = match *dir {
                dirs::REQUESTS_DIR => {
                    let configuration =
                        RawDirConfiguration::Request(Block::new(RawDirRequestConfiguration::new()));
                    hcl::to_string(&configuration)?
                }
                dirs::ENDPOINTS_DIR => {
                    let configuration = RawDirConfiguration::Endpoint(Block::new(
                        RawDirEndpointConfiguration::new(),
                    ));
                    hcl::to_string(&configuration)?
                }
                dirs::COMPONENTS_DIR => {
                    let configuration = RawDirConfiguration::Component(Block::new(
                        RawDirComponentConfiguration::new(),
                    ));
                    hcl::to_string(&configuration)?
                }
                dirs::SCHEMAS_DIR => {
                    let configuration =
                        RawDirConfiguration::Schema(Block::new(RawDirSchemaConfiguration::new()));
                    hcl::to_string(&configuration)?
                }
                _ => unreachable!(),
            };
            worktree
                .create_entry("", dir, true, content.as_bytes())
                .await?;
        }

        for dir in &OTHER_DIRS {
            self.fs.create_dir(&abs_path.join(dir)).await?;
        }

        let normalized_repo = if let Some(url) = params.repository {
            Some(normalize_git_url(&url)?)
        } else {
            None
        };

        let manifest = moss_file::toml::EditableInPlaceFileHandle::create(
            self.fs.clone(),
            abs_path.join(MANIFEST_FILE_NAME),
            ManifestModel {
                name: params
                    .name
                    .unwrap_or(defaults::DEFAULT_COLLECTION_NAME.to_string()),
                repository: normalized_repo,
            },
        )
        .await?;

        let config = TomlFileHandle::create(
            self.fs.clone(),
            &params.internal_abs_path.join(CONFIG_FILE_NAME),
            ConfigModel {
                external_path: params.external_abs_path.map(|p| p.to_owned().into()),
            },
        )
        .await?;

        if let Some(icon_path) = params.icon_path {
            // TODO: Log the error here
            let _ = SetIconService::set_icon(
                &icon_path,
                &abs_path.join(ASSETS_DIR).join(COLLECTION_ICON_FILENAME),
                ICON_SIZE,
            );
        }

        // TODO: Load environments

        Ok(Collection {
            fs: self.fs.clone(),
            services: self.services.into(),
            abs_path: params.internal_abs_path.to_owned().into(),
            worktree: Arc::new(worktree),
            storage: Arc::new(storage),
            environments: OnceCell::new(),
            manifest,
            config,
            on_did_change: EventEmitter::new(),
        })
    }
}
