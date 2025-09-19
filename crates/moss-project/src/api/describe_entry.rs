use moss_applib::AppRuntime;

use crate::{
    Project,
    models::{operations::DescribeEntryOutput, primitives::EntryId},
};

impl<R: AppRuntime> Project<R> {
    pub async fn describe_entry(
        &self,
        ctx: &R::AsyncContext,
        entry_id: EntryId,
    ) -> joinerror::Result<DescribeEntryOutput> {
        self.worktree().await.describe_entry(ctx, &entry_id).await
    }
}
