use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{
    Project,
    models::{operations::DescribeEntryOutput, primitives::EntryId},
};

impl<R: AppRuntime> Project<R> {
    pub async fn describe_entry(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        entry_id: EntryId,
    ) -> joinerror::Result<DescribeEntryOutput> {
        self.worktree()
            .await
            .describe_entry(ctx, app_delegate, &entry_id)
            .await
    }
}
