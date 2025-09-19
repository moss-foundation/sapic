use crate::{
    Project,
    models::operations::{DescribeEntryInput, DescribeEntryOutput},
};
use moss_applib::AppRuntime;

impl<R: AppRuntime> Project<R> {
    pub async fn describe_entry(
        &self,
        ctx: &R::AsyncContext,
        input: DescribeEntryInput,
    ) -> joinerror::Result<DescribeEntryOutput> {
        self.worktree().await.describe_entry(ctx, &input.id).await
    }
}
