use anyhow::Result;
use moss_applib::{
    EventMarker, ServiceMarker,
    providers::ServiceProvider,
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

use crate::{config::ConfigModel, manifest::ManifestModel, services::DynSetIconService};

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
    pub(super) manifest: JsonFileHandle<ManifestModel>,
    #[allow(dead_code)]
    pub(super) config: JsonFileHandle<ConfigModel>,

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
        let set_icon_service = self.services.get::<DynSetIconService>();
        set_icon_service.icon_path()
    }

    pub fn service<T: ServiceMarker>(&self) -> &T {
        self.services.get::<T>()
    }

    pub fn service_arc<T: ServiceMarker + Send + Sync>(&self) -> Arc<T> {
        self.services.get_arc::<T>()
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

        let set_icon_service = self.service::<DynSetIconService>();
        match params.icon_path {
            None => {}
            Some(ChangePath::Update(new_icon_path)) => {
                set_icon_service.set_icon(&new_icon_path)?;
            }
            Some(ChangePath::Remove) => {
                set_icon_service.remove_icon().await?;
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
