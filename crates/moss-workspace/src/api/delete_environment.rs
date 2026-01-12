// use crate::{
//     Workspace,
//     models::operations::{DeleteEnvironmentInput, DeleteEnvironmentOutput},
// };
// use moss_applib::AppRuntime;
//
// impl Workspace {
//     pub async fn delete_environment<R: AppRuntime>(
//         &self,
//         ctx: &R::AsyncContext,
//         input: DeleteEnvironmentInput,
//     ) -> joinerror::Result<DeleteEnvironmentOutput> {
//         self.environment_service
//             .delete_environment(ctx, &input.id)
//             .await?;
//
//         Ok(DeleteEnvironmentOutput { id: input.id })
//     }
// }
