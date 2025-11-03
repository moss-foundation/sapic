// use moss_applib::AppRuntime;
// use sapic_window::types::WorkspaceInfo;

// use crate::{app::App, models::operations::ListWorkspacesOutput};

// impl<R: AppRuntime> App<R> {
//     pub async fn list_workspaces(
//         &self,
//         _ctx: &R::AsyncContext,
//     ) -> joinerror::Result<ListWorkspacesOutput> {
//         let workspaces = self.workspace_service.list_workspaces().await?;
//         let workspaces = workspaces
//             .into_iter()
//             .map(|item| WorkspaceInfo {
//                 id: item.id,
//                 name: item.name.clone(),
//                 last_opened_at: item.last_opened_at,
//                 abs_path: item.abs_path,
//             })
//             .collect();

//         Ok(ListWorkspacesOutput(workspaces))
//     }
// }
