use anyhow::Result;
use moss_fs::{CreateOptions, FileSystem, RenameOptions};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use thiserror::Error;
use tokio::sync::RwLock;

use crate::models::{
    file::{EnvironmentFile, EnvironmentFileVariable, EnvironmentFileVariableUpdate},
    types::{VariableKind, VariableName, VariableValue},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariableCache {
    pub disabled: bool,
    pub order: Option<usize>,
    pub local_value: VariableValue,
}

impl Default for VariableCache {
    fn default() -> Self {
        Self {
            disabled: false,
            local_value: VariableValue::Null,
            order: None,
        }
    }
}

pub struct EnvironmentCache {
    pub decoded_name: String,
    pub order: Option<usize>,
    pub variables_cache: HashMap<VariableName, VariableCache>,
}

pub struct EnvironmentUpdateParams {
    pub new_file_name: Option<String>,
    pub variables_to_be_updated: HashMap<VariableName, EnvironmentFileVariableUpdate>,
    pub variables_to_be_deleted: Vec<VariableName>,
}

type VariableMap = HashMap<VariableName, EnvironmentFileVariable>;

#[derive(Error, Debug)]
pub enum EnvironmentError {
    #[error("Failed to parse environment file as JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Failed to open environment file {path}: {err}")]
    FileOpenError { err: anyhow::Error, path: PathBuf },

    #[error("Failed to create environment file {path}: {err}")]
    FileCreateError { err: anyhow::Error, path: PathBuf },

    #[error("Failed to rename environment file {old_path} to {new_path}: {err}")]
    FileRenameError {
        old_path: PathBuf,
        new_path: PathBuf,
        err: anyhow::Error,
    },

    #[error("Unknown error: {0}")]
    Unknown(anyhow::Error),
}

pub struct Environment {
    fs: Arc<dyn FileSystem>,
    path: RwLock<PathBuf>,
    variables: RwLock<VariableMap>,
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Environment")
            .field("path", &self.path)
            .field("variables", &self.variables)
            .finish()
    }
}

impl Environment {
    pub async fn new(path: PathBuf, fs: Arc<dyn FileSystem>) -> Result<Self, EnvironmentError> {
        let mut variables = HashMap::new();
        if path.exists() {
            let reader =
                fs.open_file(&path)
                    .await
                    .map_err(|err| EnvironmentError::FileOpenError {
                        err: err.into(),
                        path: path.clone(),
                    })?;
            let environment_file: EnvironmentFile = serde_json::from_reader(reader)?;
            variables = environment_file.values;
        } else {
            fs.create_file_with(
                &path,
                serde_json::to_string(&EnvironmentFile::default())?,
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: true,
                },
            )
            .await
            .map_err(|err| EnvironmentError::FileCreateError {
                err: err.into(),
                path: path.clone(),
            })?;
        }

        Ok(Self {
            fs: Arc::clone(&fs),
            path: RwLock::new(path),
            variables: RwLock::new(variables),
        })
    }

    pub fn variables(&self) -> &RwLock<VariableMap> {
        &self.variables
    }

    pub async fn update(&self, params: EnvironmentUpdateParams) -> Result<(), EnvironmentError> {
        self.update_variables(
            params.variables_to_be_updated,
            params.variables_to_be_deleted,
        )
        .await?;

        if let Some(new_file_name) = params.new_file_name {
            self.update_file_name(new_file_name).await?;
        }

        Ok(())
    }

    async fn update_file_name(&self, new_file_name: String) -> Result<(), EnvironmentError> {
        let old_path = self.path.read().await.clone();
        let new_path = old_path.with_file_name(new_file_name);
        self.fs
            .rename(
                &old_path,
                &new_path,
                RenameOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .map_err(|err| EnvironmentError::FileRenameError {
                old_path: old_path.clone(),
                new_path: new_path.clone(),
                err: err.into(),
            })?;

        *self.path.write().await = new_path;

        Ok(())
    }

    async fn update_variables(
        &self,
        variables_to_be_updated: HashMap<VariableName, EnvironmentFileVariableUpdate>,
        variables_to_be_deleted: Vec<VariableName>,
    ) -> Result<(), EnvironmentError> {
        let mut variables = self.variables.write().await;

        for (name, update) in variables_to_be_updated {
            if let Some(variable) = variables.get_mut(&name) {
                variable.update(update);
            } else {
                variables.insert(
                    name,
                    EnvironmentFileVariable {
                        kind: update.kind.unwrap_or(VariableKind::Default),
                        value: update.value.unwrap_or(VariableValue::Null),
                        desc: update.desc,
                    },
                );
            }
        }

        for name in variables_to_be_deleted {
            variables.remove(&name);
        }

        Ok(())
    }

    pub async fn clear(&self) -> Result<(), EnvironmentError> {
        self.variables.write().await.clear();
        Ok(())
    }
}
