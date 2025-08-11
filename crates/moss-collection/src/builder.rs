use crate::{
    Collection,
    config::{CONFIG_FILE_NAME, ConfigFile},
    constants::COLLECTION_ROOT_PATH,
    defaults, dirs,
    edit::CollectionEdit,
    manifest::{MANIFEST_FILE_NAME, ManifestFile},
    models::{
        primitives::EntryId,
        types::configuration::docschema::{
            RawDirComponentConfiguration, RawDirConfiguration, RawDirEndpointConfiguration,
            RawDirRequestConfiguration, RawDirSchemaConfiguration,
        },
    },
    services::{
        set_icon_service::SetIconService,
        storage_service::StorageService,
        worktree_service::{EntryMetadata, WorktreeService},
    },
};
use joinerror::ResultExt;
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_fs::{CreateOptions, FileSystem};
use moss_git::repo::RepoHandle;
use moss_git_hosting_provider::{auth::generate_auth_agent, common::GitProviderType};
use moss_hcl::Block;
use moss_keyring::KeyringClient;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::{Mutex, OnceCell};
use url::Url;

const COLLECTION_ICON_SIZE: u32 = 128;
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
    pub icon_path: Option<PathBuf>,
}

pub struct CollectionLoadParams {
    pub internal_abs_path: Arc<Path>,
}

pub struct CollectionCloneParams {
    pub git_provider_type: GitProviderType,
    pub internal_abs_path: Arc<Path>,
    pub repository: String,
}

pub struct CollectionBuilder {
    fs: Arc<dyn FileSystem>,
}

impl CollectionBuilder {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self { fs }
    }

    pub async fn load<R: AppRuntime>(
        self,
        params: CollectionLoadParams,
    ) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let storage_service: Arc<StorageService<R>> =
            StorageService::new(params.internal_abs_path.as_ref())
                .join_err::<()>("failed to create collection storage service")?
                .into();

        let worktree_service: Arc<WorktreeService<R>> = WorktreeService::new(
            params.internal_abs_path.clone(),
            self.fs.clone(),
            storage_service.clone(),
        )
        .into();

        let set_icon_service = SetIconService::new(
            params.internal_abs_path.clone(),
            self.fs.clone(),
            COLLECTION_ICON_SIZE,
        );

        let edit = CollectionEdit::new(
            self.fs.clone(),
            params.internal_abs_path.join(MANIFEST_FILE_NAME),
        );
        // TODO: Load environments

        // TODO: Load Git repo
        Ok(Collection {
            fs: self.fs.clone(),
            abs_path: params.internal_abs_path,
            edit,
            set_icon_service,
            storage_service,
            worktree_service,
            environments: OnceCell::new(),
            on_did_change: EventEmitter::new(),
            repo_handle: Arc::new(Mutex::new(None)),
        })
    }

    // TODO: A nicer functionality to have might be to help the user create a remote Git repository
    // Using Git provider API, and help them make the initial commit
    // I'll come to this question later.

    pub async fn create<R: AppRuntime>(
        self,
        ctx: &R::AsyncContext,
        params: CollectionCreateParams,
    ) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path: Arc<Path> = params
            .external_abs_path
            .clone()
            .unwrap_or(params.internal_abs_path.clone())
            .into();

        let storage_service: Arc<StorageService<R>> = StorageService::new(abs_path.as_ref())
            .join_err::<()>("failed to create collection storage service")?
            .into();

        let worktree_service: Arc<WorktreeService<R>> =
            WorktreeService::new(abs_path.clone(), self.fs.clone(), storage_service.clone()).into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), COLLECTION_ICON_SIZE);

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

        if let Some(icon_path) = params.icon_path {
            // TODO: Log the error here
            set_icon_service.set_icon(&icon_path)?;
        }

        self.fs
            .create_file_with(
                &abs_path.join(MANIFEST_FILE_NAME),
                serde_json::to_string(&ManifestFile {
                    name: params
                        .name
                        .unwrap_or(defaults::DEFAULT_COLLECTION_NAME.to_string()),
                    repository: None,
                })?
                .as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        // TODO: Add config file and others to .gitignore
        self.fs
            .create_file_with(
                &params.internal_abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ConfigFile {
                    external_path: params.external_abs_path.map(|p| p.to_path_buf()),
                })?
                .as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        let edit = CollectionEdit::new(self.fs.clone(), abs_path.join(MANIFEST_FILE_NAME));

        // TODO: Load environments

        Ok(Collection {
            fs: self.fs.clone(),
            abs_path: params.internal_abs_path.to_owned().into(),
            edit,
            set_icon_service,
            storage_service,
            worktree_service,
            environments: OnceCell::new(),
            on_did_change: EventEmitter::new(),
            repo_handle: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn clone<R: AppRuntime>(
        self,
        _ctx: &R::AsyncContext,
        params: CollectionCloneParams,
        keyring_client: Arc<dyn KeyringClient + Send + Sync>,
    ) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let repo_url = Url::parse(&params.repository)?;
        let abs_path = params.internal_abs_path.clone();

        let abs_path_clone = abs_path.clone();
        // Since git2rs is fundamentally synchronous, I suppose this is the best approach
        let join = tokio::task::spawn_blocking(move || {
            Ok(RepoHandle::clone(
                &repo_url,
                abs_path_clone.as_ref(),
                // Different git providers require different auth agent
                generate_auth_agent(keyring_client, params.git_provider_type.into())?,
            )?)
        })
        .await;

        let repo_handle = match join {
            Ok(Ok(repo_handle)) => repo_handle,
            Ok(Err(err)) => return Err(err),
            Err(err) => return Err(err.into()),
        };

        let storage_service: Arc<StorageService<R>> = StorageService::new(abs_path.as_ref())
            .join_err::<()>("failed to create collection storage service")?
            .into();

        let worktree_service: Arc<WorktreeService<R>> =
            WorktreeService::new(abs_path.clone(), self.fs.clone(), storage_service.clone()).into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), COLLECTION_ICON_SIZE);

        self.fs
            .create_file_with(
                &abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ConfigFile {
                    external_path: None,
                })?
                .as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        let edit = CollectionEdit::new(self.fs.clone(), abs_path.join(MANIFEST_FILE_NAME));

        // TODO: Load environments

        Ok(Collection {
            fs: self.fs.clone(),
            abs_path,
            edit,
            set_icon_service,
            storage_service,
            worktree_service,
            environments: OnceCell::new(),
            on_did_change: EventEmitter::new(),
            repo_handle: Arc::new(Mutex::new(Some(repo_handle))),
        })
    }
}
