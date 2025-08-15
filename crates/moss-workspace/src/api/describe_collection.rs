use moss_applib::AppRuntime;

use crate::{
    Workspace,
    models::operations::{DescribeCollectionInput, DescribeCollectionOutput},
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn describe_collection(
        &self,
        ctx: &R::AsyncContext,
        input: &DescribeCollectionInput,
    ) -> joinerror::Result<DescribeCollectionOutput> {
        let desc = self
            .collection_service
            .describe_collection(ctx, &input.id)
            .await?;
        Ok(DescribeCollectionOutput {
            name: desc.name,
            repository: desc.repository,
            repository_info: desc.repository_info,
            contributors: desc.contributors,
            current_branch: desc.current_branch,
        })
    }
}
