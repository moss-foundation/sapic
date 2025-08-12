use joinerror::ResultExt;
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_fs::{CreateOptions, FileSystem};
use moss_git::url::normalize_git_url;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;

use crate::{
    Collection,
    config::{CONFIG_FILE_NAME, ConfigFile},
    constants::COLLECTION_ROOT_PATH,
    defaults, dirs,
    edit::CollectionEdit,
    manifest::{MANIFEST_FILE_NAME, ManifestFile},
    models::primitives::{EntryClass, EntryId},
    services::{set_icon_service::SetIconService, storage_service::StorageService},
    worktree::{Worktree, entry::model::EntryModel},
};

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
    pub repository: Option<String>,
    pub icon_path: Option<PathBuf>,
}

pub struct CollectionLoadParams {
    pub internal_abs_path: Arc<Path>,
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

        let worktree_service: Arc<Worktree<R>> = Worktree::new(
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

        Ok(Collection {
            fs: self.fs.clone(),
            abs_path: params.internal_abs_path,
            edit,
            set_icon_service,
            storage_service,
            worktree: worktree_service,
            environments: OnceCell::new(),
            on_did_change: EventEmitter::new(),
        })
    }

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

        let worktree_service: Arc<Worktree<R>> =
            Worktree::new(abs_path.clone(), self.fs.clone(), storage_service.clone()).into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), COLLECTION_ICON_SIZE);

        for (dir, order) in &WORKTREE_DIRS {
            let id = EntryId::new();
            let model = match *dir {
                dirs::REQUESTS_DIR => EntryModel::from((id, EntryClass::Request)),
                dirs::ENDPOINTS_DIR => EntryModel::from((id, EntryClass::Endpoint)),
                dirs::COMPONENTS_DIR => EntryModel::from((id, EntryClass::Component)),
                dirs::SCHEMAS_DIR => EntryModel::from((id, EntryClass::Schema)),
                _ => unreachable!(),
            };

            worktree_service
                .create_dir_entry(
                    ctx,
                    dir,
                    Path::new(COLLECTION_ROOT_PATH),
                    model,
                    *order,
                    false,
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
                    repository: normalized_repo,
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
            worktree: worktree_service,
            environments: OnceCell::new(),
            on_did_change: EventEmitter::new(),
        })
    }
}
