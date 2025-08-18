pub mod batch_update_collection;
pub mod batch_update_environment;
pub mod batch_update_environment_group;
pub mod create_collection;
pub mod create_environment;
pub mod delete_collection;
pub mod delete_environment;
mod describe_collection;
pub mod describe_state;
pub mod import_collection;
pub mod stream_collections;
pub mod stream_environments;
pub mod update_collection;
pub mod update_environment;
pub mod update_environment_group;
pub mod update_state;

use moss_applib::AppRuntime;

use crate::{AnyWorkspace, models::operations::*};

#[allow(async_fn_in_trait)]
pub trait BatchUpdateCollectionOp<R: AppRuntime> {
    async fn batch_update_collection(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateCollectionInput,
    ) -> joinerror::Result<BatchUpdateCollectionOutput>;
}

pub trait AnyWorkspaceApi<R: AppRuntime>: AnyWorkspace<R> + BatchUpdateCollectionOp<R> {}
