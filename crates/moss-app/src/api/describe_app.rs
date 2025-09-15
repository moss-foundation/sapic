use joinerror::OptionExt;
use moss_applib::{AppRuntime, errors::FailedPrecondition};
use moss_user::models::types::ProfileInfo;

use crate::{
    app::App,
    models::{
        operations::DescribeAppOutput,
        types::{Configuration, WorkspaceInfo},
    },
};

impl<R: AppRuntime> App<R> {
    pub async fn describe_app(
        &self,
        _ctx: &R::AsyncContext,
    ) -> joinerror::Result<DescribeAppOutput> {
        let maybe_workspace_details =
            if let Some(workspace) = self.workspace_service.workspace().await {
                self.workspace_service
                    .workspace_details(&workspace.id)
                    .await
            } else {
                None
            };

        let active_profile = self
            .profile_service
            .active_profile()
            .await
            .ok_or_join_err::<FailedPrecondition>("no active profile to describe the app")?;
        let profile_details = self
            .profile_service
            .profile_details(&active_profile.id())
            .await
            .unwrap();
        let configuration = self.configuration_service.configuration().await;

        Ok(DescribeAppOutput {
            workspace: maybe_workspace_details.map(|details| WorkspaceInfo {
                id: details.id,
                name: details.name,
                last_opened_at: details.last_opened_at,
                abs_path: details.abs_path,
            }),
            profile: Some(ProfileInfo {
                id: active_profile.id().clone(),
                name: profile_details.name,
                accounts: profile_details.accounts,
            }),
            configuration: Configuration {
                keys: configuration
                    .keys
                    .into_iter()
                    .map(|key| key.to_string())
                    .collect(),
                contents: configuration
                    .contents
                    .into_iter()
                    .map(|(key, value)| (key.to_string(), value))
                    .collect(),
            },
        })
    }
}
