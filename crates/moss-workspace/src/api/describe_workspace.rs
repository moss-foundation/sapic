use moss_applib::AppRuntime;

use crate::{
    Workspace,
    models::{operations::DescribeWorkspaceOutput, types::Layouts},
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn describe_workspace(
        &self,
        ctx: &R::AsyncContext,
    ) -> joinerror::Result<DescribeWorkspaceOutput> {
        // HACK: cache here is a temporary solution
        let mut cache = self.storage_service.get_layout_cache(ctx).await?;

        let editor_state = self.layout_service.get_editor_layout_state(&mut cache)?;
        let sidebar_state = self.layout_service.get_sidebar_layout_state(&mut cache)?;
        let panel_state = self.layout_service.get_panel_layout_state(&mut cache)?;
        let activitybar_state = self
            .layout_service
            .get_activitybar_layout_state(&mut cache)?;

        Ok(DescribeWorkspaceOutput {
            layouts: Layouts {
                editor: editor_state,
                sidebar: Some(sidebar_state),
                panel: Some(panel_state),
                activitybar: Some(activitybar_state),
            },
        })
    }
}
