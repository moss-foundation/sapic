use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    app::App,
    models::{
        operations::DescribeAppStateOutput,
        types::{Defaults, Preferences},
    },
    services::storage_service::StorageService,
};

// TODO: We must rewrite this crap later, it's a mess

impl<R: AppRuntime> App<R> {
    pub async fn describe_app_state(
        &self,
        ctx: &R::AsyncContext,
    ) -> OperationResult<DescribeAppStateOutput> {
        let storage_service = self.services.get::<StorageService<R>>();

        let last_workspace_id =
            if let Ok(id_str) = storage_service.get_last_active_workspace(ctx).await {
                Some(id_str)
            } else {
                None
            };

        Ok(DescribeAppStateOutput {
            preferences: Preferences {
                theme: self.preferences().theme.read().await.clone(),
                locale: self.preferences().locale.read().await.clone(),
            },
            defaults: Defaults {
                theme: self.defaults().theme.clone(),
                locale: self.defaults().locale.clone(),
            },
            prev_workspace_id: last_workspace_id,
        })
    }
}
