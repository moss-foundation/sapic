use anyhow::Result;
use moss_common::models::primitives::Identifier;
use moss_fs::{CreateOptions, FileSystem, RenameOptions};
use moss_storage::workspace_storage::{
    VariableStore, entities::variable_store_entities::VariableEntity,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use thiserror::Error;
use tokio::sync::RwLock;

use crate::models::{
    file::{EnvironmentFile, EnvironmentFileVariable, EnvironmentFileVariableUpdate},
    types::{VariableKind, VariableName, VariableValue},
};

// #[derive(Error, Debug)]
// pub enum EnvironmentError {
//     #[error("Failed to parse environment file as JSON: {0}")]
//     JsonParseError(#[from] serde_json::Error),

//     #[error("Failed to open environment file {path}: {err}")]
//     FileOpenError { err: anyhow::Error, path: PathBuf },

//     #[error("Failed to create environment file {path}: {err}")]
//     FileCreateError { err: anyhow::Error, path: PathBuf },

//     #[error("Failed to rename environment file {old_path} to {new_path}: {err}")]
//     FileRenameError {
//         old_path: PathBuf,
//         new_path: PathBuf,
//         err: anyhow::Error,
//     },

//     #[error("Unknown error: {0}")]
//     Unknown(anyhow::Error),
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct VariableCache {
//     pub disabled: bool,
//     pub order: Option<usize>,
//     pub local_value: VariableValue,
// }

// impl Default for VariableCache {
//     fn default() -> Self {
//         Self {
//             disabled: false,
//             local_value: VariableValue::Null,
//             order: None,
//         }
//     }
// }

// impl TryFrom<VariableStateEntity> for VariableCache {
//     type Error = anyhow::Error;

//     fn try_from(value: VariableStateEntity) -> Result<Self, Self::Error> {
//         Ok(Self {
//             disabled: value.disabled,
//             local_value: VariableValue::try_from(value.local_value)?,
//             order: value.order,
//         })
//     }
// }

// pub struct EnvironmentCache {
//     pub decoded_name: String,
//     pub order: Option<usize>,
//     pub variables_cache: HashMap<VariableName, VariableCache>,
// }

// pub struct EnvironmentUpdateParams {
//     pub new_file_name: Option<String>,
//     pub variables_to_be_updated: HashMap<VariableName, EnvironmentFileVariableUpdate>,
//     pub variables_to_be_deleted: Vec<VariableName>,
// }

#[derive(Debug, Clone)]
pub struct VariableParams {
    pub disabled: bool,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub id: Identifier,
    pub kind: Option<VariableKind>,
    pub global_value: Option<VariableValue>,
    pub local_value: Option<VariableValue>,
    pub desc: Option<String>,
    pub order: Option<usize>,
    pub params: VariableParams,
}

type VariableMap = HashMap<VariableName, Variable>;

pub struct Environment {
    fs: Arc<dyn FileSystem>,
    abs_path: Arc<Path>,
    variables: RwLock<VariableMap>,
    next_variable_id: Arc<AtomicUsize>,
    variable_store: Arc<dyn VariableStore>,
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Environment")
            .field("path", &self.abs_path)
            .field("variables", &self.variables)
            .finish()
    }
}

impl Environment {
    pub async fn new(
        abs_path: Arc<Path>,
        fs: Arc<dyn FileSystem>,
        environment_store: Arc<dyn VariableStore>,
        next_variable_id: Arc<AtomicUsize>,
    ) -> Result<Self> {
        debug_assert!(abs_path.is_file());
        debug_assert!(abs_path.is_absolute());

        if !abs_path.exists() {
            let environment_file = EnvironmentFile::default();
            fs.create_file_with(
                &abs_path,
                serde_json::to_string(&environment_file)?.as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: true,
                },
            )
            .await?;
        }

        if abs_path
            .extension()
            .map(|ext| ext != "json")
            .unwrap_or(false)
        {
            return Err(anyhow::anyhow!(
                "Environment file must have a .json extension"
            ));
        }

        let reader = fs.open_file(&abs_path).await?;
        let environment_file: EnvironmentFile = serde_json::from_reader(reader)?;

        let _variables_cache = environment_store.list_variables()?;

        let mut variables = HashMap::new();
        for (name, value) in environment_file.values {
            variables.insert(
                name,
                Variable {
                    id: Identifier::new(&next_variable_id),
                    kind: value.kind,
                    global_value: value.value,
                    local_value: None, // TODO: restore this value from cache
                    desc: value.desc,
                    order: None, // TODO: restore this value from cache
                    params: VariableParams {
                        disabled: true, // TODO: restore this value from cache
                    },
                },
            );
        }

        Ok(Self {
            fs,
            abs_path,
            variables: RwLock::new(variables),
            next_variable_id,
            variable_store: environment_store,
        })

        // let mut variables = HashMap::new();
        // if path.exists() {
        //     let reader =
        //         fs.open_file(&path)
        //             .await
        //             .map_err(|err| EnvironmentError::FileOpenError {
        //                 err: err.into(),
        //                 path: path.clone(),
        //             })?;
        //     let environment_file: EnvironmentFile = serde_json::from_reader(reader)?;
        //     variables = environment_file.values;
        // } else {
        //     fs.create_file_with(
        //         &path,
        //         serde_json::to_string(&EnvironmentFile::default())?,
        //         CreateOptions {
        //             overwrite: false,
        //             ignore_if_exists: true,
        //         },
        //     )
        //     .await
        //     .map_err(|err| EnvironmentError::FileCreateError {
        //         err: err.into(),
        //         path: path.clone(),
        //     })?;
        // }

        // Ok(Self {
        //     fs: Arc::clone(&fs),
        //     path: RwLock::new(path),
        //     variables: RwLock::new(variables),
        // })

        // todo!()
    }

    pub fn variables(&self) -> &RwLock<VariableMap> {
        &self.variables
    }

    // pub async fn update(&self, params: EnvironmentUpdateParams) -> Result<(), EnvironmentError> {
    //     // self.update_variables(
    //     //     params.variables_to_be_updated,
    //     //     params.variables_to_be_deleted,
    //     // )
    //     // .await?;

    //     // if let Some(new_file_name) = params.new_file_name {
    //     //     self.update_file_name(new_file_name).await?;
    //     // }

    //     // Ok(())

    //     todo!()
    // }

    async fn update_file_name(&self, new_file_name: String) -> Result<()> {
        // let old_path = self.path.read().await.clone();
        // let new_path = old_path.with_file_name(new_file_name);
        // self.fs
        //     .rename(
        //         &old_path,
        //         &new_path,
        //         RenameOptions {
        //             overwrite: true,
        //             ignore_if_exists: false,
        //         },
        //     )
        //     .await
        //     .map_err(|err| EnvironmentError::FileRenameError {
        //         old_path: old_path.clone(),
        //         new_path: new_path.clone(),
        //         err: err.into(),
        //     })?;

        // *self.path.write().await = new_path;

        // Ok(())

        todo!()
    }

    async fn update_variables(
        &self,
        variables_to_be_updated: HashMap<VariableName, EnvironmentFileVariableUpdate>,
        variables_to_be_deleted: Vec<VariableName>,
    ) -> Result<()> {
        // let mut variables = self.variables.write().await;

        // for (name, update) in variables_to_be_updated {
        //     if let Some(variable) = variables.get_mut(&name) {
        //         variable.update(update);
        //     } else {
        //         variables.insert(
        //             name,
        //             EnvironmentFileVariable {
        //                 kind: update.kind.unwrap_or(VariableKind::Default),
        //                 value: update.value.unwrap_or(VariableValue::Null),
        //                 desc: update.desc,
        //             },
        //         );
        //     }
        // }

        // for name in variables_to_be_deleted {
        //     variables.remove(&name);
        // }

        // Ok(())

        todo!()
    }

    pub async fn clear(&self) -> Result<()> {
        self.variables.write().await.clear();
        Ok(())
    }
}
