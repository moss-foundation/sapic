use anyhow::Result;
use moss_applib::{
    AppRuntime, EventMarker,
    subscription::{Event, EventEmitter},
};
use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_environment::{environment::Environment, models::primitives::EnvironmentId};
use moss_file::json::JsonFileHandle;
use moss_fs::FileSystem;
use moss_git::url::normalize_git_url;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;

use crate::{
    config::ConfigModel,
    manifest::ManifestModel,
    services::{
        set_icon_service::SetIconService, storage_service::StorageService,
        worktree_service::WorktreeService,
    },
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

    pub(super) set_icon_service: SetIconService,
    pub(super) storage_service: Arc<StorageService<R>>,
    pub(super) worktree_service: Arc<WorktreeService<R>>,

    #[allow(dead_code)]
    pub(super) environments: OnceCell<EnvironmentMap<R>>,
    pub(super) manifest: JsonFileHandle<ManifestModel>,
    #[allow(dead_code)]
    pub(super) config: JsonFileHandle<ConfigModel>,

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

    pub async fn modify(&self, params: CollectionModifyParams) -> Result<()> {
        if params.name.is_some() || params.repository.is_some() {
            let normalized_repo = if let Some(ChangeString::Update(url)) = &params.repository {
                Some(ChangeString::Update(normalize_git_url(url)?))
            } else {
                params.repository
            };

            self.manifest
                .edit(
                    |model| {
                        model.name = params.name.unwrap_or(model.name.clone());
                        match normalized_repo {
                            None => {}
                            Some(ChangeString::Remove) => {
                                model.repository = None;
                            }
                            Some(ChangeString::Update(url)) => {
                                model.repository = Some(url);
                            }
                        }
                        Ok(())
                    },
                    |model| {
                        serde_json::to_string(model).map_err(|err| {
                            anyhow::anyhow!("Failed to serialize JSON file: {}", err)
                        })
                    },
                )
                .await?;
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

        Ok(())
    }

    pub async fn manifest(&self) -> ManifestModel {
        self.manifest.model().await
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
}

#[cfg(any(test, feature = "integration-tests"))]
impl<R: AppRuntime> Collection<R> {
    pub fn set_icon_service(&self) -> &SetIconService {
        &self.set_icon_service
    }

    pub fn storage_service(&self) -> Arc<StorageService<R>> {
        self.storage_service.clone()
    }

    pub fn worktree_service(&self) -> Arc<WorktreeService<R>> {
        self.worktree_service.clone()
    }
}
