use joinerror::ResultExt;
use json_patch::PatchOperation;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem, RenameOptions, error::FsResultExt};
use moss_hcl::HclResultExt;
use serde_json::Value as JsonValue;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::watch;

use crate::{configuration::SourceFile, environment::EnvironmentPath};

struct EnvironmentEditingState {
    abs_path: PathBuf,
    edit: JsonEdit,
}

pub(super) struct EnvironmentEditing {
    fs: Arc<dyn FileSystem>,
    state: EnvironmentEditingState,
    abs_path_tx: watch::Sender<EnvironmentPath>,
}

impl EnvironmentEditing {
    pub fn new(fs: Arc<dyn FileSystem>, abs_path_tx: watch::Sender<EnvironmentPath>) -> Self {
        let abs_path = abs_path_tx.borrow().to_path_buf();
        debug_assert!(abs_path.is_absolute());

        Self {
            fs,
            abs_path_tx,
            state: EnvironmentEditingState {
                abs_path,
                edit: JsonEdit::new(),
            },
        }
    }

    pub async fn rename(&mut self, new_name: &str) -> joinerror::Result<()> {
        let parent = self.abs_path_tx.borrow().parent.clone();
        let new_abs_path = EnvironmentPath::new(parent.join(new_name))
            .join_err::<()>("failed to create new environment path")?;

        self.fs
            .rename(
                &self.state.abs_path,
                &new_abs_path.full_path,
                RenameOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await?;

        self.state.abs_path = new_abs_path.full_path.clone();

        let _ = self.abs_path_tx.send(new_abs_path);

        Ok(())
    }

    pub async fn edit(
        &mut self,
        params: &[(PatchOperation, EditOptions)],
    ) -> joinerror::Result<()> {
        let rdr = self
            .fs
            .open_file(&self.state.abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to open file: {}", self.state.abs_path.display())
            })?;

        let mut value: JsonValue = hcl::from_reader(rdr).join_err::<()>("failed to parse json")?;

        self.state
            .edit
            .apply(&mut value, params)
            .join_err::<()>("failed to apply patches")?;

        let parsed: SourceFile = serde_json::from_value(value)?;
        let content = hcl::to_string(&parsed).join_err::<()>("failed to serialize json")?;
        self.fs
            .create_file_with(
                &self.state.abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| {
                format!("failed to write file: {}", self.state.abs_path.display())
            })?;

        Ok(())
    }
}
