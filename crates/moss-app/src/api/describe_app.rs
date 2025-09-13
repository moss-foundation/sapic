use moss_applib::AppRuntime;
use moss_logging::session;

use crate::{
    app::App,
    models::{operations::DescribeAppOutput, types::Configuration},
};

impl<R: AppRuntime> App<R> {
    pub async fn describe_app(
        &self,
        ctx: &R::AsyncContext,
    ) -> joinerror::Result<DescribeAppOutput> {
        let last_workspace_id = match self.storage_service.get_last_active_workspace(ctx).await {
            Ok(id) => Some(id),
            Err(err) => {
                session::error!(format!(
                    "failed to restore last active workspace: {}",
                    err.to_string()
                ));

                None
            }
        };
        let active_profile = self.profile_service.active_profile().await;
        let configuration = self.configuration_service.configuration().await;

        Ok(DescribeAppOutput {
            opened: last_workspace_id,
            profile: Some(active_profile.id().clone()),

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
