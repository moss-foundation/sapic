use anyhow::Result;
use moss_applib::{
    AppRuntime, ServiceMarker,
    providers::{ServiceMap, ServiceProvider},
    subscription::EventEmitter,
};
use moss_file::json::JsonFileHandle;
use moss_fs::FileSystem;
use moss_git::url::normalize_git_url;
use moss_hcl::Block;
use std::{
    any::TypeId,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;

use crate::{
    Collection,
    config::{CONFIG_FILE_NAME, ConfigModel},
    constants::COLLECTION_ROOT_PATH,
    defaults, dirs,
    manifest::{MANIFEST_FILE_NAME, ManifestModel},
    models::{
        primitives::EntryId,
        types::configuration::docschema::{
            RawDirComponentConfiguration, RawDirConfiguration, RawDirEndpointConfiguration,
            RawDirRequestConfiguration, RawDirSchemaConfiguration,
        },
    },
    services::{DynSetIconService, DynWorktreeService, worktree_service::EntryMetadata},
};

const OTHER_DIRS: [&str; 2] = [dirs::ASSETS_DIR, dirs::ENVIRONMENTS_DIR];

const WORKTREE_DIRS: [(&str, isize); 4] = [
    (dirs::REQUESTS_DIR, 0),
    (dirs::ENDPOINTS_DIR, 1),
    (dirs::COMPONENTS_DIR, 2),
    (dirs::SCHEMAS_DIR, 3),
];

pub struct CollectionCreateParams {
    pub name: Option<String>,
    pub internal_abs_path: Arc<Path>,
    pub external_abs_path: Option<Arc<Path>>,
    pub repository: Option<String>,
    pub icon_path: Option<PathBuf>,
}

pub struct CollectionLoadParams {
    pub internal_abs_path: Arc<Path>,
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

    pub fn with_service<T: ServiceMarker + Send + Sync>(
        mut self,
        service: impl Into<Arc<T>>,
    ) -> Self {
        self.services.insert(TypeId::of::<T>(), service.into());
        self
    }

    pub async fn load<R: AppRuntime>(self, params: CollectionLoadParams) -> Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let manifest = JsonFileHandle::load(
            self.fs.clone(),
            &params.internal_abs_path.join(MANIFEST_FILE_NAME),
        )
        .await?;

        let config = JsonFileHandle::load(
            self.fs.clone(),
            &params.internal_abs_path.join(CONFIG_FILE_NAME),
        )
        .await?;

        // TODO: Load environments

        Ok(Collection {
            fs: self.fs.clone(),
            services: self.services.into(),
            abs_path: params.internal_abs_path,
            environments: OnceCell::new(),
            manifest,
            config,
            on_did_change: EventEmitter::new(),
        })
    }

    pub async fn create<R: AppRuntime>(
        self,
        ctx: &R::AsyncContext,
        params: CollectionCreateParams,
    ) -> Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path: Arc<Path> = params
            .external_abs_path
            .clone()
            .unwrap_or(params.internal_abs_path.clone())
            .into();

        let services: ServiceProvider = self.services.into();
        let worktree_service = services.get::<DynWorktreeService<R>>();

        for (dir, order) in &WORKTREE_DIRS {
            let id = EntryId::new();
            let configuration = match *dir {
                dirs::REQUESTS_DIR => {
                    RawDirConfiguration::Request(Block::new(RawDirRequestConfiguration::new(&id)))
                }
                dirs::ENDPOINTS_DIR => {
                    RawDirConfiguration::Endpoint(Block::new(RawDirEndpointConfiguration::new(&id)))
                }
                dirs::COMPONENTS_DIR => RawDirConfiguration::Component(Block::new(
                    RawDirComponentConfiguration::new(&id),
                )),
                dirs::SCHEMAS_DIR => {
                    RawDirConfiguration::Schema(Block::new(RawDirSchemaConfiguration::new(&id)))
                }
                _ => unreachable!(),
            };

            worktree_service
                .create_dir_entry(
                    ctx,
                    &id,
                    dir,
                    Path::new(COLLECTION_ROOT_PATH),
                    configuration,
                    EntryMetadata {
                        order: *order,
                        expanded: false,
                    },
                )
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

        let manifest = JsonFileHandle::create(
            self.fs.clone(),
            &abs_path.join(MANIFEST_FILE_NAME),
            ManifestModel {
                name: params
                    .name
                    .unwrap_or(defaults::DEFAULT_COLLECTION_NAME.to_string()),
                repository: normalized_repo,
            },
        )
        .await?;

        let config = JsonFileHandle::create(
            self.fs.clone(),
            &params.internal_abs_path.join(CONFIG_FILE_NAME),
            ConfigModel {
                external_path: params.external_abs_path.map(|p| p.to_owned().into()),
            },
        )
        .await?;

        if let Some(icon_path) = params.icon_path {
            let set_icon_service = services.get::<DynSetIconService>();
            // TODO: Log the error here
            set_icon_service.set_icon(&icon_path)?;
        }

        // TODO: Load environments

        Ok(Collection {
            fs: self.fs.clone(),
            services,
            abs_path: params.internal_abs_path.to_owned().into(),
            environments: OnceCell::new(),
            manifest,
            config,
            on_did_change: EventEmitter::new(),
        })
    }
}
