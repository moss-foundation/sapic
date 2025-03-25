use anyhow::Result;
use moss_fs::{CreateOptions, FileSystem};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::{OnceCell, RwLock};

use crate::models::{
    file::EnvironmentFile,
    types::{VariableInfo, VariableName},
};

pub struct EnvironmentMetadata {
    pub name: String,
}

type VariableMap = HashMap<VariableName, VariableInfo>;

pub struct Environment {
    fs: Arc<dyn FileSystem>,
    path: PathBuf,
    variables: OnceCell<RwLock<VariableMap>>,
}

impl Environment {
    pub async fn new(path: PathBuf, fs: Arc<dyn FileSystem>) -> Result<Self> {
        if !path.exists() {
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
            variables: OnceCell::new(),
        })
    }

    pub async fn variables(&self) -> Result<&RwLock<VariableMap>> {
        let variables = self
            .variables
            .get_or_try_init(|| async move {
                let reader = self.fs.open_file(&self.path).await?;

                let environment_file: EnvironmentFile = serde_json::from_reader(reader)?;

                Ok::<_, anyhow::Error>(RwLock::new(environment_file.values))
            })
            .await?;

        Ok(variables)
    }
}
