use crate::{
    config::ConfigModel,
    constants::COLLECTION_ICON_FILENAME,
    dirs::ASSETS_DIR,
    manifest::{ManifestModel, ManifestModelDiff},
    services::set_icon::{SetIconService, constants::ICON_SIZE},
};
use anyhow::Result;
use moss_applib::{
    EventMarker, ServiceMarker,
    providers::ServiceProvider,
    subscription::{Event, EventEmitter},
};
use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_environment::{environment::Environment, models::primitives::EnvironmentId};
use moss_file::toml::TomlFileHandle;
use moss_fs::{FileSystem, RemoveOptions};
use moss_git::url::normalize_git_url;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;

pub struct EnvironmentItem {
    pub id: EnvironmentId,
    pub name: String,
    pub inner: Environment,
}

type EnvironmentMap = HashMap<EnvironmentId, Arc<EnvironmentItem>>;

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

pub struct Collection {
    #[allow(dead_code)]
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) abs_path: Arc<Path>,
    pub(super) services: ServiceProvider,
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
    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    pub fn external_path(&self) -> Option<&Arc<Path>> {
        unimplemented!()
    }

    pub fn icon_path(&self) -> Option<PathBuf> {
        let path = self
            .abs_path
            .join(ASSETS_DIR)
            .join(COLLECTION_ICON_FILENAME);

        path.exists().then_some(path)
    }

    pub fn service<T: ServiceMarker>(&self) -> &T {
        self.services.get::<T>()
    }

    pub fn service_arc<T: ServiceMarker + Send + Sync>(&self) -> Arc<T> {
        self.services.get_arc::<T>()
    }

    pub async fn modify(&self, params: CollectionModifyParams) -> Result<()> {
        if params.name.is_some() || params.repository.is_some() {
            let normalized_repo = if let Some(ChangeString::Update(url)) = params.repository {
                Some(ChangeString::Update(normalize_git_url(&url)?))
            } else {
                None
            };

            self.manifest
                .edit(ManifestModelDiff {
                    name: params.name,
                    repository: normalized_repo,
                })
                .await?;
        }

        match params.icon_path {
            None => {}
            Some(ChangePath::Update(new_icon_path)) => {
                SetIconService::set_icon(
                    &new_icon_path,
                    &self
                        .abs_path
                        .join(ASSETS_DIR)
                        .join(COLLECTION_ICON_FILENAME),
                    ICON_SIZE,
                )?;
            }
            Some(ChangePath::Remove) => {
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
