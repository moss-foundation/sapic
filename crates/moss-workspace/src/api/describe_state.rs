use std::collections::HashMap;

use moss_common::api::OperationResult;
use moss_db::primitives::AnyValue;
use moss_storage::primitives::segkey::SegKeyBuf;
use tauri::Runtime as TauriRuntime;

use crate::{
    Workspace,
    models::operations::DescribeStateOutput,
    services::{layout_service::LayoutService, storage_service::StorageService},
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn describe_state(&self) -> OperationResult<DescribeStateOutput> {
        let layout = self.services.get::<LayoutService>();
        let storage = self.services.get::<StorageService>();

        // HACK: cache here is a temporary solution
        let mut cache = storage
            .get_layout_cache()?
            .map(|(segkey, value)| (segkey, value))
            .collect::<HashMap<SegKeyBuf, AnyValue>>();

        let editor_state = layout.get_editor_layout_state(&mut cache)?;
        let sidebar_state = layout.get_sidebar_layout_state(&mut cache)?;
        let panel_state = layout.get_panel_layout_state(&mut cache)?;
        let activitybar_state = layout.get_activitybar_layout_state(&mut cache)?;

        Ok(DescribeStateOutput {
            editor: editor_state,
            sidebar: Some(sidebar_state),
            panel: Some(panel_state),
            activitybar: Some(activitybar_state),
        })
    }
}
