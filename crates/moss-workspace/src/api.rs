pub mod activate_environment;
pub mod archive_project;
pub mod batch_update_environment;
pub mod batch_update_environment_group;
pub mod batch_update_project;
pub mod create_environment;
pub mod create_project;
pub mod delete_environment;
pub mod delete_project;
pub mod describe_project;
pub mod export_project;
pub mod import_project;
pub mod list_changes;
pub mod stream_environments;
pub mod stream_projects;
pub mod unarchive_project;
pub mod update_environment;
pub mod update_environment_group;
pub mod update_project;

use moss_applib::AppRuntime;

use crate::{AnyWorkspace, models::operations::*};

#[allow(async_fn_in_trait)]
pub trait BatchUpdateProjectOp<R: AppRuntime> {
    async fn batch_update_project(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateProjectInput,
    ) -> joinerror::Result<BatchUpdateProjectOutput>;
}

pub trait AnyWorkspaceApi<R: AppRuntime>: AnyWorkspace<R> + BatchUpdateProjectOp<R> {}
