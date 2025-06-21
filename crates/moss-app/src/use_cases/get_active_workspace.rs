use crate::{
    app::App,
    context::{AnyAppContext, AppContext},
};

// impl<R: TauriRuntime> App<R> {
//     pub async fn get_active_workspace(&self) -> Option<(WorkspaceReadGuard<'_, R>, WorkspaceContext<R>)> {
//         self.service::<WorkspaceService<R>>()
//             .active_workspace(self.app_handle.clone())
//             .await
//     }
// }
