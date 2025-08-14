use crate::{
    models::{
        events::StreamCollectionsEvent, operations::StreamCollectionsOutput, types::BranchInfo,
    },
    workspace::Workspace,
};
use futures::StreamExt;
use moss_applib::AppRuntime;
use moss_git::repo::RepoHandle;

use std::sync::{Arc, Mutex};
use tauri::ipc::Channel as TauriChannel;

impl<R: AppRuntime> Workspace<R> {
    pub async fn stream_collections(
        &self,
        ctx: &R::AsyncContext,
        channel: TauriChannel<StreamCollectionsEvent>,
    ) -> joinerror::Result<StreamCollectionsOutput> {
        let stream = self.collection_service.list_collections(ctx).await;
        tokio::pin!(stream);

        let mut total_returned = 0;
        while let Some(collection) = stream.next().await {
            // TODO: It might be better to separate the sending of information fetched from HTTP
            // from the main streaming, which will make the application more responsive
            // Right now the latency from HTTP requests slows down this operation quite a lot

            let repo_handle = self
                .collection(&collection.id)
                .await
                .map(|c| c.repo_handle());
            let branch_info = if let Some(repo_handle) = repo_handle {
                fetch_current_branch_info(repo_handle)
                    .await
                    .unwrap_or_else(|e| {
                        // TODO: Tell the frontend that we failed to fetch current branch info
                        println!("failed to fetch current branch info: {}", e.to_string());
                        None
                    })
            } else {
                None
            };

            let event = StreamCollectionsEvent {
                id: collection.id,
                name: collection.name,
                order: collection.order,
                expanded: collection.expanded,
                repository: collection.repository,
                branch: branch_info,
                picture_path: collection.icon_path,
            };

            if let Err(e) = channel.send(event) {
                println!("Error sending collection event: {:?}", e); // TODO: log error
            } else {
                total_returned += 1;
            }
        }

        Ok(StreamCollectionsOutput { total_returned })
    }
}

// async fn fetch_remote_repo_info(
//     repository: Option<String>,
//     github_client: Arc<GitHubClient>,
//     gitlab_client: Arc<GitLabClient>,
// ) -> (Option<RepositoryInfo>, Vec<Contributor>) {
//     if let Some(Ok(repo_ref)) = repository.as_ref().map(|x| GitUrl::parse(&x)) {
//         match repo_ref.domain.as_str() {
//             // FIXME: Handle custom GitLab domains
//             "github.com" => (
//                 github_client.repository_info(&repo_ref).await.ok(),
//                 github_client
//                     .contributors(&repo_ref)
//                     .await
//                     .unwrap_or_default(),
//             ),
//             "gitlab.com" => (
//                 gitlab_client.repository_info(&repo_ref).await.ok(),
//                 gitlab_client
//                     .contributors(&repo_ref)
//                     .await
//                     .unwrap_or_default(),
//             ),
//             _ => (None, Vec::new()),
//         }
//     } else {
//         (None, Vec::new())
//     }
// }

async fn fetch_current_branch_info(
    repo_handle: Arc<Mutex<Option<RepoHandle>>>,
) -> joinerror::Result<Option<BranchInfo>> {
    let result = tokio::task::spawn_blocking(move || {
        let repo_handle_lock = repo_handle.lock()?;
        let repo_handle_ref = repo_handle_lock.as_ref();
        if repo_handle_ref.is_none() {
            return Ok(None);
        }
        let repo_handle_ref = repo_handle_ref.unwrap();
        // TODO: Support custom origin name? We assume it's `origin` now, which we use when we create a repo

        let current_branch = repo_handle_ref.current_branch()?;

        // git fetch
        repo_handle_ref.fetch(Some("origin"))?;

        // Compare local with remote state
        let (ahead, behind) = repo_handle_ref.compare_with_remote_branch(&current_branch)?;

        Ok(Some(BranchInfo {
            name: current_branch,
            ahead,
            behind,
        }))
    })
    .await?;

    match result {
        Ok(Some(info)) => Ok(Some(info)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}
