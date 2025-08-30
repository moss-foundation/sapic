use moss_git_hosting_provider::{GitHostingProvider, common::GitUrl, models::types::Contributor};
use std::sync::Arc;

// pub(crate) async fn fetch_contributors(
//     repo_ref: &GitUrl,
//     client: Arc<dyn GitHostingProvider>,
// ) -> joinerror::Result<Vec<Contributor>> {
//     // INFO: In the future we might support non-VCS contributors
//     match client.contributors(repo_ref).await {
//         Ok(contributors) => Ok(contributors),
//         Err(e) => {
//             // TODO: Tell the frontend provider API call fails
//             println!("git provider api call fails: {}", e);
//             Ok(Vec::new())
//         }
//     }
// }
