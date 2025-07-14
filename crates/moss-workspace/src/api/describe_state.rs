use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    Workspace,
    models::operations::DescribeStateOutput,
    services::{DynLayoutService, DynStorageService},
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn describe_state(
        &self,
        ctx: &R::AsyncContext,
    ) -> OperationResult<DescribeStateOutput> {
        let layout = self.services.get::<DynLayoutService<R>>();
        let storage = self.services.get::<DynStorageService<R>>();

        // HACK: cache here is a temporary solution
        let mut cache = storage.get_layout_cache(ctx).await?;

        let editor_state = layout.get_editor_layout_state(ctx, &mut cache).await?;
        let sidebar_state = layout.get_sidebar_layout_state(ctx, &mut cache).await?;
        let panel_state = layout.get_panel_layout_state(ctx, &mut cache).await?;
        let activitybar_state = layout.get_activitybar_layout_state(ctx, &mut cache).await?;

        Ok(DescribeStateOutput {
            editor: editor_state,
            sidebar: Some(sidebar_state),
            panel: Some(panel_state),
            activitybar: Some(activitybar_state),
        })
    }
}
