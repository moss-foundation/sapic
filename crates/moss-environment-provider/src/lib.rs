use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use joinerror::ResultExt;
use moss_applib::AppRuntime;
use moss_common::continue_if_err;
use moss_environment::{
    Environment,
    builder::{CreateEnvironmentParams, EnvironmentBuilder, EnvironmentLoadParams},
    models::types::AddVariableParams,
};
use moss_fs::FileSystem;
use moss_storage::common::VariableStore;
use tokio::sync::mpsc::UnboundedSender;

pub struct EnvironmentProviderCreateParams {
    pub name: String,
    pub color: Option<String>,
    pub variables: Vec<AddVariableParams>,
}

#[derive(Clone)]
pub struct EnvironmentProvider {
    fs: Arc<dyn FileSystem>,
    abs_path: PathBuf,
}

impl std::fmt::Debug for EnvironmentProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnvironmentProvider")
            .field("abs_path", &self.abs_path)
            .finish()
    }
}

impl EnvironmentProvider {
    pub fn new(fs: Arc<dyn FileSystem>, abs_path: PathBuf) -> Self {
        Self { fs, abs_path }
    }

    pub async fn scan<R: AppRuntime>(
        &self,
        store: Arc<dyn VariableStore<R::AsyncContext>>,
        tx: UnboundedSender<Environment<R>>,
    ) -> joinerror::Result<()> {
        println!("scanning environment provider: {}", self.abs_path.display());
        let mut read_dir = self.fs.read_dir(&self.abs_path).await.map_err(|err| {
            joinerror::Error::new::<()>(format!(
                "failed to read directory {} : {}",
                self.abs_path.display(),
                err
            ))
        })?;

        while let Some(entry) = read_dir.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                continue;
            }

            let maybe_environment = EnvironmentBuilder::new(self.fs.clone())
                .load::<R>(
                    store.clone(),
                    EnvironmentLoadParams {
                        abs_path: entry.path(),
                    },
                )
                .await;
            let environment = continue_if_err!(maybe_environment, |err| {
                println!(
                    "failed to load environment {}: {}",
                    entry.path().display(),
                    err
                );
            });

            tx.send(environment).ok();
        }

        Ok(())
    }

    pub async fn create<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        store: Arc<dyn VariableStore<R::AsyncContext>>,
        params: EnvironmentProviderCreateParams,
    ) -> joinerror::Result<Environment<R>> {
        let environment = EnvironmentBuilder::new(self.fs.clone())
            .create::<R>(
                ctx,
                store,
                CreateEnvironmentParams {
                    name: params.name.clone(),
                    abs_path: &self.abs_path,
                    color: params.color,
                    variables: params.variables,
                },
            )
            .await?;

        Ok(environment)
    }

    pub async fn delete<R: AppRuntime>(&self) -> joinerror::Result<()> {
        Ok(())
    }
}
