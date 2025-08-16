use futures::StreamExt;
use moss_applib::AppRuntime;
use moss_git_hosting_provider::{
    GitHostingProvider,
    common::GitUrl,
    github::client::GitHubClient,
    gitlab::client::GitLabClient,
    models::types::{Contributor, RepositoryInfo},
};
use std::sync::Arc;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    models::{events::StreamCollectionsEvent, operations::StreamCollectionsOutput},
    workspace::Workspace,
};

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
            // FIXME: Not sure if we should put the logic for fetching git provider API here

            let (repository_info, contributors) = if let Some(Ok(repo_ref)) =
                collection.repository.as_ref().map(|x| GitUrl::parse(&x))
            {
                fetch_remote_repo_info(
                    &repo_ref,
                    self.github_client.clone(),
                    self.gitlab_client.clone(),
                )
                .await
            } else {
                (None, Vec::new())
            };

            if let Err(e) = channel.send(StreamCollectionsEvent {
                id: collection.id,
                name: collection.name,
                order: collection.order,
                expanded: collection.expanded,
                repository: collection.repository,
                repository_info,
                contributors,
                icon_path: collection.icon_path,
            }) {
                println!("Error sending collection event: {:?}", e); // TODO: log error
            } else {
                total_returned += 1;
            }
        }

        Ok(StreamCollectionsOutput { total_returned })
    }
}

async fn fetch_remote_repo_info(
    repo_ref: &GitUrl,
    github_client: Arc<GitHubClient>,
    gitlab_client: Arc<GitLabClient>,
) -> (Option<RepositoryInfo>, Vec<Contributor>) {
    match repo_ref.domain.as_str() {
        // FIXME: Handle custom GitLab domains
        "github.com" => (
            github_client.repository_info(repo_ref).await.ok(),
            github_client
                .contributors(repo_ref)
                .await
                .unwrap_or_default(),
        ),
        "gitlab.com" => (
            gitlab_client.repository_info(repo_ref).await.ok(),
            gitlab_client
                .contributors(repo_ref)
                .await
                .unwrap_or_default(),
        ),
        _ => (None, Vec::new()),
    }
}
