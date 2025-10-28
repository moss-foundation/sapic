pub mod adapters;
pub mod application_storage;
pub mod provider;
pub mod workspace_storage;

use derive_more::Deref;
use joinerror::Error;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use rustc_hash::FxHashMap;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::{path::Path, str::FromStr, sync::Arc};
use tokio::sync::OnceCell;

use crate::provider::{AppStorageBackendProvider, StorageBackendProvider};

const DEFAULT_DB_FILENAME: &str = "state.sqlite3";

pub enum Scope {
    Global,
    Workspace(Arc<String>),
}

pub trait Storage: Send + Sync {
    // async fn get(&self, key: &str) -> joinerror::Result<String>;
    // async fn set(&self, key: &str, value: &str) -> joinerror::Result<()>;
    // async fn delete(&self, key: &str) -> joinerror::Result<()>;
}

pub struct AppStorage {
    provider: Arc<dyn StorageBackendProvider>,
}

impl Storage for AppStorage {}

impl AppStorage {
    pub async fn new(globals_dir: &Path) -> joinerror::Result<Arc<Self>> {
        let provider = AppStorageBackendProvider::new(globals_dir).await?;

        provider.application().await?;

        Ok(Self {
            provider: Arc::new(provider),
        }
        .into())
    }
}

#[derive(Deref, Clone)]
pub struct GlobalStorage(Arc<dyn Storage>);

impl dyn Storage {
    pub fn global<R: AppRuntime>(delegate: &AppDelegate<R>) -> Arc<dyn Storage> {
        delegate.global::<GlobalStorage>().0.clone()
    }

    pub fn set_global<R: AppRuntime>(delegate: &AppDelegate<R>, v: Arc<dyn Storage>) {
        delegate.set_global(GlobalStorage(v));
    }
}
