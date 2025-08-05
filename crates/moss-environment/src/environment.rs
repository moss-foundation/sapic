use derive_more::Deref;
use joinerror::{Error, ResultExt};
use moss_applib::AppRuntime;
use moss_fs::{FileSystem, RenameOptions, model_registry::GlobalModelRegistry};
use moss_hcl::hcl_to_json;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::{
    AnyEnvironment, DescribeEnvironment, ModifyEnvironmentParams,
    models::types::VariableInfo,
    services::{
        metadata_service::MetadataService, sync_service::SyncService,
        variable_service::VariableService,
    },
    utils,
};
#[derive(Debug, Deref)]
pub(super) struct EnvironmentPath {
    parent: PathBuf,
    name: String,

    #[deref]
    path: Arc<Path>,
}

impl EnvironmentPath {
    pub fn new(abs_path: Arc<Path>) -> joinerror::Result<Self> {
        debug_assert!(abs_path.is_absolute());

        let parent = abs_path
            .parent()
            .ok_or_else(|| joinerror::Error::new::<()>("environment path must have a parent"))?;

        let name = abs_path
            .file_name()
            .ok_or_else(|| joinerror::Error::new::<()>("environment path must have a name"))?;

        Ok(Self {
            parent: parent.to_path_buf(),
            name: name.to_string_lossy().to_string(),
            path: abs_path,
        })
    }
}

pub(super) struct EnvironmentState {
    pub abs_path: EnvironmentPath,
}

pub struct Environment<R: AppRuntime> {
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) model_registry: Arc<GlobalModelRegistry>,
    pub(super) state: RwLock<EnvironmentState>,
    pub(super) metadata_service: MetadataService,
    pub(super) sync_service: Arc<SyncService>,
    pub(super) variable_service: VariableService<R>,
}

unsafe impl<R: AppRuntime> Send for Environment<R> {}
unsafe impl<R: AppRuntime> Sync for Environment<R> {}

impl<R: AppRuntime> AnyEnvironment<R> for Environment<R> {
    async fn abs_path(&self) -> Arc<Path> {
        self.state.read().await.abs_path.path.clone()
    }

    async fn name(&self) -> joinerror::Result<String> {
        let filename = self.state.read().await.abs_path.name.clone();
        utils::parse_file_name(&filename).map_err(|err| {
            Error::new::<()>(format!("failed to parse environment file name: {}", err))
        })
    }

    async fn color(&self) -> Option<String> {
        None // TODO: hardcoded for now
    }

    async fn describe(&self) -> joinerror::Result<DescribeEnvironment> {
        let metadata = self
            .metadata_service
            .describe(&self.abs_path().await)
            .await?;

        let var_items = self.variable_service.list().await;
        let mut variables = Vec::with_capacity(var_items.len());
        for (id, variable) in var_items {
            let global_value = match hcl_to_json(&variable.global_value) {
                Ok(value) => value,
                Err(err) => {
                    println!("failed to convert global value expression: {}", err);
                    continue;
                }
            };
            let local_value = match hcl_to_json(&variable.local_value) {
                Ok(value) => value,
                Err(err) => {
                    println!("failed to convert local value expression: {}", err);
                    continue;
                }
            };

            variables.push(VariableInfo {
                id,
                name: variable.name,
                global_value,
                local_value,
                disabled: variable.options.disabled,
                order: variable.order,
                desc: variable.desc,
            });
        }

        Ok(DescribeEnvironment {
            id: metadata.id,
            color: metadata.color,
            name: self.name().await?,
            variables,
        })
    }

    async fn modify(&self, params: ModifyEnvironmentParams) -> joinerror::Result<()> {
        if let Some(new_name) = params.name {
            self.rename(new_name).await?;
        }

        let abs_path = self.state.read().await.abs_path.path.clone();

        self.variable_service
            .batch_add(&abs_path, params.vars_to_add)
            .await?;
        self.variable_service
            .batch_remove(&abs_path, params.vars_to_delete)
            .await?;

        // TODO: update metadata(color)

        // TODO: we'll handle file system synchronization in the background a bit later,
        // so we can respond to the frontend faster.

        self.sync_service.save(&abs_path).await?;

        Ok(())
    }
}

impl<R: AppRuntime> Environment<R> {
    async fn rename(&self, new_name: String) -> joinerror::Result<()> {
        let new_file_name = utils::format_file_name(&new_name);
        let mut state = self.state.write().await;
        let current_abs_path = state.abs_path.path.clone();
        let new_abs_path: Arc<Path> = state.abs_path.parent.join(new_file_name).into();
        let environment_path =
            EnvironmentPath::new(new_abs_path.clone()).join_err_with::<()>(|| {
                format!(
                    "failed to create environment path {}",
                    new_abs_path.display()
                )
            })?;

        self.fs
            .rename(
                &current_abs_path,
                &new_abs_path,
                RenameOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .map_err(|err| Error::new::<()>(format!("failed to rename file: {}", err)))?;

        self.model_registry
            .rekey(&current_abs_path, new_abs_path.clone())
            .await;

        state.abs_path = environment_path;

        Ok(())
    }
}
