use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{
    Project,
    models::{operations::DescribeResourceOutput, primitives::ResourceId},
};

impl<R: AppRuntime> Project<R> {
    pub async fn describe_resource(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        entry_id: ResourceId,
    ) -> joinerror::Result<DescribeResourceOutput> {
        self.worktree()
            .await
            .describe_entry(ctx, app_delegate, &entry_id)
            .await
    }
}
