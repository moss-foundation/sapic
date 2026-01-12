// use moss_app_delegate::AppDelegate;
// use moss_applib::AppRuntime;
// use sapic_ipc::ValidationResultExt;
// use validator::Validate;
//
// use crate::{
//     environment::CreateEnvironmentItemParams,
//     models::operations::{CreateEnvironmentInput, CreateEnvironmentOutput},
//     workspace::Workspace,
// };
//
// impl Workspace {
//     pub async fn create_environment<R: AppRuntime>(
//         &self,
//         ctx: &R::AsyncContext,
//         _app_delegate: AppDelegate<R>,
//         input: CreateEnvironmentInput,
//     ) -> joinerror::Result<CreateEnvironmentOutput> {
//         input.validate().join_err_bare()?;
//
//         let result = self
//             .environment_service
//             .create_environment(
//                 ctx,
//                 CreateEnvironmentItemParams {
//                     project_id: input.project_id,
//                     name: input.name.clone(),
//                     order: input.order,
//                     color: input.color.clone(),
//                     variables: input.variables,
//                 },
//             )
//             .await?;
//
//         Ok(CreateEnvironmentOutput {
//             id: result.id,
//             project_id: result.project_id.map(|id| id.into()),
//             name: result.display_name,
//             order: result.order,
//             color: result.color,
//             abs_path: result.abs_path.to_path_buf(),
//         })
//     }
// }
