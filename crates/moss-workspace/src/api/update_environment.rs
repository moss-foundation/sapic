// use moss_applib::AppRuntime;
// use sapic_ipc::ValidationResultExt;
// use validator::Validate;
//
// use crate::{
//     models::operations::{UpdateEnvironmentInput, UpdateEnvironmentOutput},
//     workspace::Workspace,
// };
//
// impl Workspace {
//     pub async fn update_environment<R: AppRuntime>(
//         &self,
//         ctx: &R::AsyncContext,
//         input: UpdateEnvironmentInput,
//     ) -> joinerror::Result<UpdateEnvironmentOutput> {
//         input.validate().join_err_bare()?;
//
//         let id = input.inner.id.clone();
//         self.environment_service
//             .update_environment(ctx, input.inner)
//             .await?;
//
//         Ok(UpdateEnvironmentOutput { id })
//     }
// }
