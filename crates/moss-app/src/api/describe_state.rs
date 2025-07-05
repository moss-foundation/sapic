use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App,
    models::{
        operations::DescribeAppStateOutput,
        types::{Defaults, Preferences},
    },
    services::workspace_service::WorkspaceService,
};

// TODO: We must rewrite this crap later, it's a mess

impl<R: TauriRuntime> App<R> {
    pub async fn describe_state(&self) -> OperationResult<DescribeAppStateOutput> {
        let workspace_service = self.services.get::<WorkspaceService<R>>();

        // HACK: This is a hack to get the last workspace name
        let active_workspace_lock = workspace_service.workspace().await;

        let last_workspace_name = &active_workspace_lock.as_ref().map(|active_workspace| {
            active_workspace
                .abs_path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        });

        Ok(DescribeAppStateOutput {
            preferences: Preferences {
                theme: self.preferences().theme.read().await.clone(),
                locale: self.preferences().locale.read().await.clone(),
            },
            defaults: Defaults {
                theme: self.defaults().theme.clone(),
                locale: self.defaults().locale.clone(),
            },
            last_workspace: last_workspace_name.clone(), // Some("TestWorkspace".to_string())
        })
    }
}
