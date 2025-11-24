use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, errors::FailedPrecondition};
use moss_user::models::types::ProfileInfo;
use sapic_base::workspace::types::WorkspaceInfo;
use sapic_runtime::{app::settings_storage::SettingScope, globals::GlobalSettingsStorage};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

use crate::{
    models::{operations::DescribeAppOutput, types::Configuration},
    window::OldSapicWindow,
};

impl<R: AppRuntime> OldSapicWindow<R> {
    pub async fn describe_app(
        &self,
        _ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
    ) -> joinerror::Result<DescribeAppOutput> {
        let maybe_workspace_details =
            if let Some(workspace) = self.workspace_service.workspace().await {
                self.workspace_service
                    .workspace_details(&workspace.id())
                    .await
                    .unwrap()
            } else {
                None
            };

        dbg!(&maybe_workspace_details);

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

        let settings_storage = GlobalSettingsStorage::get(app_delegate);
        let configuration: HashMap<String, JsonValue> = HashMap::from_iter(
            settings_storage
                .values(&SettingScope::User)
                .await
                .into_iter(),
        );

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
                keys: configuration.keys().map(|key| key.clone()).collect(),
                contents: configuration
                    .into_iter()
                    .map(|(key, value)| (key, value))
                    .collect(),
            },
        })
    }
}
