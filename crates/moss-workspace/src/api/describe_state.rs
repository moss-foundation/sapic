use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{Workspace, models::operations::DescribeStateOutput};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn describe_state(&self) -> OperationResult<DescribeStateOutput> {
        let mut txn = self.storage.begin_read()?;

        let editor_state = self.layout.editor_state(&mut txn)?;
        let sidebar_state = self.layout.sidebar_state(&mut txn)?;
        let panel_state = self.layout.panel_state(&mut txn)?;
        let activitybar_state = self.layout.activitybar_state(&mut txn)?;

        Ok(DescribeStateOutput {
            editor: editor_state,
            sidebar: Some(sidebar_state),
            panel: Some(panel_state),
            activitybar: Some(activitybar_state),
        })
    }
}
