use joinerror::Error;
use moss_fs::FileSystem;
use moss_storage2::KvStorage;
use sapic_base::environment::types::primitives::EnvironmentId;
use sapic_core::context::AnyAsyncContext;
use std::{path::PathBuf, sync::Arc};

use crate::{
    environment::Environment, errors::ErrorEnvironmentNotFound, models::types::AddVariableParams,
};

pub struct CreateEnvironmentParams {
    pub name: String,
    pub color: Option<String>,
    pub variables: Vec<AddVariableParams>,
}

pub struct EnvironmentLoadParams {
    pub abs_path: PathBuf,
}

pub struct EnvironmentBuilder {
    workspace_id: Arc<String>,
    fs: Arc<dyn FileSystem>,
    storage: Arc<dyn KvStorage>,
    env_id: EnvironmentId,
}

impl EnvironmentBuilder {
    pub fn new(
        workspace_id: Arc<String>,
        fs: Arc<dyn FileSystem>,
        storage: Arc<dyn KvStorage>,
        id: EnvironmentId,
    ) -> Self {
        Self {
            workspace_id,
            fs,
            storage,
            env_id: id,
        }
    }

    pub async fn create(
        self,
        _ctx: &dyn AnyAsyncContext,
        _params: CreateEnvironmentParams,
    ) -> joinerror::Result<Environment> {
        Ok(Environment {
            _id: self.env_id,
            _fs: self.fs,
            _storage: self.storage,
            _workspace_id: self.workspace_id,
        })
    }

    pub async fn load(self, params: EnvironmentLoadParams) -> joinerror::Result<Environment> {
        debug_assert!(params.abs_path.is_absolute());

        if !params.abs_path.exists() {
            return Err(Error::new::<ErrorEnvironmentNotFound>(
                params.abs_path.display().to_string(),
            ));
        }

        Ok(Environment {
            _id: self.env_id,
            _fs: self.fs,
            _storage: self.storage,
            _workspace_id: self.workspace_id,
        })
    }
}
