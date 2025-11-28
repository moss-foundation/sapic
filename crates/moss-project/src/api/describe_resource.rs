use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{
    Project,
    models::{operations::DescribeResourceOutput, primitives::ResourceId},
};

impl Project {
    pub async fn describe_resource<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        resource_id: ResourceId,
    ) -> joinerror::Result<DescribeResourceOutput> {
        self.worktree()
            .await
            .describe_entry(ctx, app_delegate, &resource_id)
            .await
    }
}
