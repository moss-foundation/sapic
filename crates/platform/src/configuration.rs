use async_trait::async_trait;
use joinerror::ResultExt;
use json_patch::{PatchOperation, ReplaceOperation, jsonptr::PointerBuf};
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem};
use sapic_base::configuration::ConfigurationModel;
use sapic_core::context::AnyAsyncContext;
use sapic_system::configuration::SettingsStore;
use serde_json::{Map, Value as JsonValue};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

pub struct FsSettingsStorage {
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    model: RwLock<ConfigurationModel>,
    edits: RwLock<JsonEdit>,
}

impl FsSettingsStorage {
    pub async fn new(
        ctx: &dyn AnyAsyncContext,
        fs: Arc<dyn FileSystem>,
        abs_path: PathBuf,
    ) -> joinerror::Result<Self> {
        let parsed = Self::load_internal(ctx, fs.as_ref(), &abs_path).await?;

        Ok(Self {
            fs,
            abs_path,
            model: RwLock::new(ConfigurationModel {
                keys: parsed.keys().map(|key| key.clone()).collect(),
                contents: parsed,
            }),
            edits: RwLock::new(JsonEdit::new()),
        })
    }

    async fn reload(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<()> {
        let parsed = Self::load_internal(ctx, self.fs.as_ref(), &self.abs_path).await?;
        *self.model.write().await = ConfigurationModel {
            keys: parsed.keys().map(|key| key.clone()).collect(),
            contents: parsed,
        };

        Ok(())
    }

    async fn load_internal(
        ctx: &dyn AnyAsyncContext,
        fs: &dyn FileSystem,
        source: &Path,
    ) -> joinerror::Result<Map<String, JsonValue>> {
        if !source.exists() {
            return Ok(Map::new());
        }

        let rdr = fs.open_file(ctx, &source).await.join_err_with::<()>(|| {
            format!("failed to open profile settings file: {}", source.display())
        })?;

        Ok(serde_json::from_reader(rdr).join_err_with::<()>(|| {
            format!(
                "failed to parse profile settings file: {}",
                source.display()
            )
        })?)
    }
}

#[async_trait]
impl SettingsStore for FsSettingsStorage {
    async fn values(&self) -> Map<String, JsonValue> {
        self.model.read().await.contents.clone()
    }

    async fn get_value(&self, key: &str) -> Option<JsonValue> {
        self.model.read().await.contents.get(key).cloned()
    }

    async fn update_value(
        &self,
        ctx: &dyn AnyAsyncContext,
        key: &str,
        value: JsonValue,
    ) -> joinerror::Result<()> {
        if !self.abs_path.exists() {
            let parent = self.abs_path.parent().unwrap();
            self.fs.create_dir_all(ctx, parent).await?;
            self.fs
                .create_file_with(
                    ctx,
                    &self.abs_path,
                    b"{}",
                    CreateOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await?;
        }

        let mut raw = self.model.write().await.raw();
        self.edits
            .write()
            .await
            .apply(
                &mut raw,
                &[(
                    PatchOperation::Replace(ReplaceOperation {
                        path: unsafe { PointerBuf::new_unchecked(format!("/{}", key)) },
                        value: value.clone(),
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: true,
                    },
                )],
            )
            .join_err::<()>("failed to edit settings file")?;

        let content = serde_json::to_string(&raw).join_err::<()>("failed to serialize json")?;

        self.fs
            .create_file_with(
                ctx,
                &self.abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| format!("failed to write file: {}", self.abs_path.display()))?;

        self.reload(ctx).await
    }

    async fn remove_value(&self, _key: &str) -> joinerror::Result<Option<JsonValue>> {
        unimplemented!()
    }
}
