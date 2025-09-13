use moss_applib::AppRuntime;

use crate::{
    app::App,
    models::{
        operations::DescribeAppOutput,
        types::{Defaults, Preferences},
    },
};

impl<R: AppRuntime> App<R> {
    pub async fn describe_profile(
        &self,
        ctx: &R::AsyncContext,
    ) -> joinerror::Result<DescribeAppOutput> {
        // let last_workspace_id =
        //     if let Ok(id_str) = self.storage_service.get_last_active_workspace(ctx).await {
        //         Some(id_str)
        //     } else {
        //         None
        //     };

        // Ok(DescribeAppStateOutput {
        //     preferences: Preferences {
        //         theme: self.preferences().theme.read().await.clone(),
        //         locale: self.preferences().locale.read().await.clone(),
        //     },
        //     defaults: Defaults {
        //         theme: self.defaults().theme.clone(),
        //         locale: self.defaults().locale.clone(),
        //     },
        //     prev_workspace_id: last_workspace_id,
        // })

        todo!()
    }
}
