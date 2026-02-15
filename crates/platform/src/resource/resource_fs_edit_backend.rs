use async_trait::async_trait;
use joinerror::{Error, OptionExt, ResultExt};
use json_patch::PatchOperation;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem, RenameOptions};
use moss_hcl::HclResultExt;
use sapic_base::resource::{
    constants::*,
    errors::{ErrorNameAlreadyExists, ErrorNameInvalid},
    manifest::EntryModel,
};
use sapic_core::context::AnyAsyncContext;
use sapic_system::resource::{ResourceEditBackend, ResourceEditParams};
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};
use tokio::sync::{RwLock, watch};

pub struct ResourceFsEditBackend {
    fs: Arc<dyn FileSystem>,
    path_tx: watch::Sender<Arc<Path>>,
    is_dir: bool,
    edits: RwLock<JsonEdit>,
}

impl ResourceFsEditBackend {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        path_tx: watch::Sender<Arc<Path>>,
        is_dir: bool,
    ) -> Arc<Self> {
        Arc::new(Self {
            fs,
            path_tx,
            is_dir,
            edits: RwLock::new(JsonEdit::new()),
        })
    }

    pub async fn rename(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
        from: &Path,
        to: &Path,
    ) -> joinerror::Result<()> {
        debug_assert!(abs_path.is_absolute());
        debug_assert!(from.is_relative());
        debug_assert!(to.is_relative());

        // On Windows and macOS, file/directory names are case-preserving but insensitive
        // If the from and to path differs only with different casing of the filename
        // The rename should still succeed

        let abs_from = abs_path.join(from);
        let abs_to = abs_path.join(to);

        let old_name_lower = from
            .file_name()
            .map(|name| name.to_string_lossy().to_lowercase())
            .ok_or_join_err::<ErrorNameInvalid>("invalid old file name")?;
        let new_name_lower = to
            .file_name()
            .map(|name| name.to_string_lossy().to_lowercase())
            .ok_or_join_err::<ErrorNameInvalid>("invalid new file name")?;

        let recasing_only =
            old_name_lower == new_name_lower && abs_from.parent() == abs_to.parent();

        if abs_to.exists() && !recasing_only {
            return Err(Error::new::<ErrorNameAlreadyExists>(format!(
                "entry already exists: {}",
                abs_to.display()
            )));
        }

        self.fs
            .rename(
                ctx,
                &abs_from,
                &abs_to,
                RenameOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await?;

        let new_path: Arc<Path> = to.into();

        let _ = self.path_tx.send(new_path);

        Ok(())
    }

    async fn patch(
        &self,
        ctx: &dyn AnyAsyncContext,
        patches: &[(PatchOperation, EditOptions)],
    ) -> joinerror::Result<()> {
        let abs_path = self.path_tx.borrow().join(if self.is_dir {
            DIR_CONFIG_FILENAME
        } else {
            ITEM_CONFIG_FILENAME
        });

        let rdr = self
            .fs
            .open_file(ctx, &abs_path)
            .await
            .join_err_with::<()>(|| format!("failed to open file: {}", abs_path.display()))?;

        let mut value: JsonValue = hcl::from_reader(rdr).join_err::<()>("failed to parse json")?;
        self.edits
            .write()
            .await
            .apply(&mut value, patches)
            .join_err::<()>("failed to apply patches")?;

        let parsed: EntryModel = serde_json::from_value(value)?;
        let content = hcl::to_string(&parsed).join_err::<()>("failed to serialize json")?;
        self.fs
            .create_file_with(
                ctx,
                &abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| format!("failed to write file: {}", abs_path.display()))?;

        Ok(())
    }
}

#[async_trait]
impl ResourceEditBackend for ResourceFsEditBackend {
    async fn edit<'a>(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ResourceEditParams<'a>,
    ) -> joinerror::Result<()> {
        if let Some(params) = params.name {
            self.rename(ctx, params.abs_path, params.from, params.to)
                .await
                .join_err::<()>("failed to rename resource")?;
        }

        if !params.patches.is_empty() {
            self.patch(ctx, params.patches)
                .await
                .join_err::<()>("failed to patch resource")?;
        }

        Ok(())
    }
}
