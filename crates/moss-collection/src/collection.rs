use anyhow::Result;
use moss_applib::{
    EventMarker, ServiceMarker,
    providers::ServiceProvider,
    subscription::{Event, EventEmitter},
};
use moss_common::api::Change;
use moss_environment::environment::Environment;
use moss_file::toml::TomlFileHandle;
use moss_fs::{FileSystem, RemoveOptions};
use moss_git::url::normalize_git_url;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;
use uuid::Uuid;

use crate::{
    config::ConfigModel,
    constants::COLLECTION_ICON_FILENAME,
    dirs::ASSETS_DIR,
    manifest::{ManifestModel, ManifestModelDiff},
    services::set_icon::{SetIconService, constants::ICON_SIZE},
    worktree::Worktree,
};

pub struct EnvironmentItem {
    pub id: Uuid,
    pub name: String,
    pub inner: Environment,
}

type EnvironmentMap = HashMap<Uuid, Arc<EnvironmentItem>>;

#[derive(Debug, Clone)]
pub enum OnDidChangeEvent {
    Toggled(bool),
}

impl EventMarker for OnDidChangeEvent {}

pub struct ModifyParams {
    pub name: Option<String>,
    pub repository: Option<Change<String>>,
    pub icon: Option<Change<PathBuf>>,
}

pub struct Collection {
    #[allow(dead_code)]
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) services: ServiceProvider,
    pub(super) worktree: Arc<Worktree>,
    pub(super) abs_path: Arc<Path>,
    // pub(super) storage: Arc<dyn CollectionStorage>,
    #[allow(dead_code)]
    pub(super) environments: OnceCell<EnvironmentMap>,
    pub(super) manifest: moss_file::toml::EditableInPlaceFileHandle<ManifestModel>,
    #[allow(dead_code)]
    pub(super) config: TomlFileHandle<ConfigModel>,

    pub(super) on_did_change: EventEmitter<OnDidChangeEvent>,
}

#[rustfmt::skip]
impl Collection {
    pub fn on_did_change(&self) -> Event<OnDidChangeEvent> { self.on_did_change.event() }
}

impl Collection {
    pub fn service<T: ServiceMarker>(&self) -> &T {
        self.services.get::<T>()
    }

    pub fn service_arc<T: ServiceMarker + Send + Sync>(&self) -> Arc<T> {
        self.services.get_arc::<T>()
    }

    pub async fn modify(&self, params: ModifyParams) -> Result<()> {
        let repo_change = match params.repository {
            None => None,
            Some(Change::Update(url)) => Some(Change::Update(normalize_git_url(&url)?)),
            Some(Change::Remove) => Some(Change::Remove),
        };

        if params.name.is_some() || repo_change.is_some() {
            self.manifest
                .edit(ManifestModelDiff {
                    name: params.name,
                    repository: repo_change,
                })
                .await?;
        }

        match params.icon {
            None => {}
            Some(Change::Update(new_icon_path)) => {
                SetIconService::set_icon(
                    &new_icon_path,
                    &self
                        .abs_path
                        .join(ASSETS_DIR)
                        .join(COLLECTION_ICON_FILENAME),
                    ICON_SIZE,
                )?;
            }
            Some(Change::Remove) => {
                self.fs
                    .remove_file(
                        &self
                            .abs_path
                            .join(ASSETS_DIR)
                            .join(COLLECTION_ICON_FILENAME),
                        RemoveOptions {
                            recursive: false,
                            ignore_if_not_exists: true,
                        },
                    )
                    .await?;
            }
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

    // #[allow(dead_code)]
    // pub(super) fn storage(&self) -> &Arc<dyn CollectionStorage> {
    //     &self.storage
    // }

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
