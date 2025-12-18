// use futures::StreamExt;
// use moss_applib::AppRuntime;
// use moss_logging::session;
// use tauri::ipc::Channel as TauriChannel;

// use crate::{
//     models::{events::StreamProjectsEvent, operations::StreamProjectsOutput},
//     workspace::Workspace,
// };

// impl Workspace {
//     pub async fn stream_projects<R: AppRuntime>(
//         &self,
//         ctx: &R::AsyncContext,
//         channel: TauriChannel<StreamProjectsEvent>,
//     ) -> joinerror::Result<StreamProjectsOutput> {
//         let stream = self.project_service.list_projects::<R>(ctx).await;
//         tokio::pin!(stream);

//         let mut total_returned = 0;

//         // OPTIMIZE: Right now `stream_projects` need to do provider API calls, which is slow
//         // We should consider streaming vcs summary from a different channel
//         //
//         // @brutusyhy, yes, absolutely, I've added a separate function to fetch VCS summary, so
//         // we can stream VCS summary in a tauri channel on the background instead of returning it
//         // as a part of the stream DTO.

//         while let Some(desc) = stream.next().await {
//             let event = StreamProjectsEvent {
//                 id: desc.id,
//                 name: desc.name,
//                 order: desc.order,
//                 expanded: desc.expanded,
//                 branch: desc.vcs.map(|vcs| vcs.branch),
//                 icon_path: desc.icon_path,
//                 archived: desc.archived,
//             };

//             if let Err(e) = channel.send(event) {
//                 session::error!(format!(
//                     "failed to send project event through tauri channel: {}",
//                     e.to_string()
//                 ));
//             } else {
//                 total_returned += 1;
//             }
//         }

//         Ok(StreamProjectsOutput { total_returned })
//     }
// }
