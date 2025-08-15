use anyhow::Result;
use joinerror::ResultExt;
use json_patch::{
    AddOperation, PatchOperation, RemoveOperation, ReplaceOperation, jsonptr::PointerBuf,
};
use moss_applib::{
    AppRuntime, EventMarker,
    subscription::{Event, EventEmitter},
};
use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_edit::json::EditOptions;
use moss_environment::{environment::Environment, models::primitives::EnvironmentId};
use moss_fs::{FileSystem, FsResultExt};
use moss_git::url::normalize_git_url;

use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;

use crate::{
    DescribeCollection,
    edit::CollectionEdit,
    manifest::{MANIFEST_FILE_NAME, ManifestFile},
    services::{
        git_service::GitService, set_icon_service::SetIconService, storage_service::StorageService,
    },
    worktree::Worktree,
};

pub struct EnvironmentItem<R: AppRuntime> {
    pub id: EnvironmentId,
    pub name: String,
    pub inner: Environment<R>,
}

type EnvironmentMap<R> = HashMap<EnvironmentId, Arc<EnvironmentItem<R>>>;

#[derive(Debug, Clone)]
pub enum OnDidChangeEvent {
    Toggled(bool),
}

impl EventMarker for OnDidChangeEvent {}

pub struct CollectionModifyParams {
    pub name: Option<String>,
    pub repository: Option<ChangeString>,
    pub icon_path: Option<ChangePath>,
}

pub struct Collection<R: AppRuntime> {
    #[allow(dead_code)]
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) abs_path: Arc<Path>,
    pub(super) edit: CollectionEdit,
    pub(super) worktree: Arc<Worktree<R>>,
    pub(super) set_icon_service: SetIconService,
    pub(super) storage_service: Arc<StorageService<R>>,
    pub(super) git_service: Arc<GitService>,

    #[allow(dead_code)]
    pub(super) environments: OnceCell<EnvironmentMap<R>>,

    pub(super) on_did_change: EventEmitter<OnDidChangeEvent>,
}

#[rustfmt::skip]
impl<R: AppRuntime> Collection<R> {
    pub fn on_did_change(&self) -> Event<OnDidChangeEvent> { self.on_did_change.event() }
}

impl<R: AppRuntime> Collection<R> {
    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    pub fn external_path(&self) -> Option<&Arc<Path>> {
        unimplemented!()
    }

    pub fn icon_path(&self) -> Option<PathBuf> {
        self.set_icon_service.icon_path()
    }
    pub async fn describe(&self) -> joinerror::Result<DescribeCollection> {
        let manifest_path = self.abs_path.join(MANIFEST_FILE_NAME);

        let rdr = self
            .fs
            .open_file(&manifest_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to open manifest file: {}", manifest_path.display())
            })?;

        let manifest: ManifestFile = serde_json::from_reader(rdr).join_err_with::<()>(|| {
            format!("failed to parse manifest file: {}", manifest_path.display())
        })?;

        Ok(DescribeCollection {
            name: manifest.name,
            repository: manifest.repository,
        })
    }

    pub async fn modify(&self, params: CollectionModifyParams) -> joinerror::Result<()> {
        let mut patches = Vec::new();

        if let Some(new_name) = params.name {
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/name") },
                    value: JsonValue::String(new_name),
                }),
                EditOptions {
                    create_missing_segments: false,
                    ignore_if_not_exists: false,
                },
            ));
        }

        match params.repository {
            Some(ChangeString::Update(url)) => {
                let normalized_url = normalize_git_url(&url)?;
                patches.push((
                    PatchOperation::Add(AddOperation {
                        path: unsafe { PointerBuf::new_unchecked("/repository") },
                        value: JsonValue::String(normalized_url),
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
            }
            Some(ChangeString::Remove) => {
                patches.push((
                    PatchOperation::Remove(RemoveOperation {
                        path: unsafe { PointerBuf::new_unchecked("/repository") },
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: true,
                    },
                ));
            }
            None => {}
        }

        match params.icon_path {
            None => {}
            Some(ChangePath::Update(new_icon_path)) => {
                self.set_icon_service.set_icon(&new_icon_path)?;
            }
            Some(ChangePath::Remove) => {
                self.set_icon_service.remove_icon().await?;
            }
        }
        self.edit
            .edit(&patches)
            .await
            .join_err::<()>("failed to edit collection")?;

        Ok(())
    }
    pub async fn environments(&self) -> Result<&EnvironmentMap<R>> {
        let result = self
            .environments
            .get_or_try_init(|| async move {
                let environments = HashMap::new();
                Ok::<_, anyhow::Error>(environments)
            })
            .await?;

        Ok(result)
    }

    pub fn git_service(&self) -> Arc<GitService> {
        self.git_service.clone()
    }
}

#[cfg(any(test, feature = "integration-tests"))]
impl<R: AppRuntime> Collection<R> {
    pub fn db(&self) -> &Arc<dyn moss_storage::CollectionStorage<R::AsyncContext>> {
        self.storage_service.storage()
    }
}
