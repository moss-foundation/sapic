// use moss_app_delegate::AppDelegate;
// use moss_applib::AppRuntime;

// use crate::{app::App, models::operations::DeleteWorkspaceInput};

// impl<R: AppRuntime> App<R> {
//     pub async fn delete_workspace(
//         &self,
//         ctx: &R::AsyncContext,
//         app_delegate: &AppDelegate<R>,
//         input: &DeleteWorkspaceInput,
//     ) -> joinerror::Result<()> {
//         self.workspace_service
//             .delete_workspace(ctx, app_delegate, &input.id)
//             .await?;

//         Ok(())
//     }
// }
