use anyhow::Result;
use moss_fs::{CreateOptions, FileSystem};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::models::{
    file::{EnvironmentFile, EnvironmentFileVariable},
    types::{VariableName, VariableValue},
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

type VariableMap = HashMap<VariableName, EnvironmentFileVariable>;

pub struct Environment {
    fs: Arc<dyn FileSystem>,
    path: PathBuf,
    variables: RwLock<VariableMap>,
}

impl Environment {
    pub async fn new(path: PathBuf, fs: Arc<dyn FileSystem>) -> Result<Self> {
        let mut variables = HashMap::new();
        if path.exists() {
            let reader = fs.open_file(&path).await?;
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
            .await?;
        }

        Ok(Self {
            fs: Arc::clone(&fs),
            path,
            variables: RwLock::new(variables),
        })
    }

    pub fn variables(&self) -> &RwLock<VariableMap> {
        &self.variables
    }
}
