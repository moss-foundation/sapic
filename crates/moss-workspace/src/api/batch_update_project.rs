use moss_applib::AppRuntime;
use sapic_ipc::ValidationResultExt;
use validator::Validate;

use crate::{
    models::operations::{BatchUpdateProjectInput, BatchUpdateProjectOutput},
    workspace::Workspace,
};

impl Workspace {
    pub async fn batch_update_project<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateProjectInput,
    ) -> joinerror::Result<BatchUpdateProjectOutput> {
        input.validate().join_err_bare()?;

        let mut ids = Vec::new();
        for item in input.items {
            let id = item.id.clone();
            self.project_service
                .update_project::<R>(ctx, &id, item)
                .await?;

            ids.push(id);
        }

        Ok(BatchUpdateProjectOutput { ids })
    }
}
