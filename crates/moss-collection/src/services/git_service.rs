// use joinerror::OptionExt;
// use moss_fs::{FileSystem, RemoveOptions};
// use moss_git::{models::types::BranchInfo, repo::FileStatus, repository::Repository};
// use std::{
//     collections::HashMap,
//     path::PathBuf,
//     sync::{Arc, Mutex},
// };

// use crate::git::GitClient;

// pub struct GitService {
//     /// All operations over the `RepoHandle` will be wrapped inside a `spawn_blocking` closure
//     /// to avoid blocking the main thread
//     pub(super) repository: Arc<Mutex<Option<Repository>>>,
//     client: GitClient,
// }

// impl GitService {
//     pub fn new(repository: Option<Repository>, client: GitClient) -> Self {
//         Self {
//             repository: Arc::new(Mutex::new(repository)),
//             client,
//         }
//     }

//     // Sometimes objects might be set as readonly, preventing them from being deleted
//     // we will need to recursively set all files in .git/objects as writable
//     pub async fn dispose(&self, fs: Arc<dyn FileSystem>) -> joinerror::Result<()> {
//         let repo_handle = self.repository.lock()?.take();
//         if repo_handle.is_none() {
//             return Ok(());
//         }
//         let repo_handle = repo_handle.unwrap();
//         let path = repo_handle.path().to_path_buf();
//         drop(repo_handle);

//         let mut folders = vec![path.join("objects")];

//         while let Some(folder) = folders.pop() {
//             let mut read_dir = fs.read_dir(&folder).await?;
//             while let Some(entry) = read_dir.next_entry().await? {
//                 if entry.file_type().await?.is_dir() {
//                     folders.push(entry.path());
//                 }
//                 let mut perms = entry.metadata().await?.permissions();
//                 perms.set_readonly(false);
//                 tokio::fs::set_permissions(&entry.path(), perms).await?;
//             }
//         }

//         fs.remove_dir(
//             &path,
//             RemoveOptions {
//                 recursive: true,
//                 ignore_if_not_exists: true,
//             },
//         )
//         .await?;

//         Ok(())
//     }

//     #[allow(dead_code)] // TODO: Remove if we will not use this method
//     pub async fn has_repo(&self) -> joinerror::Result<bool> {
//         let repo_handle_clone = self.repository.clone();
//         let join = tokio::task::spawn_blocking(move || {
//             let repo_handle_lock = repo_handle_clone.lock()?;
//             return Ok(repo_handle_lock.is_some());
//         })
//         .await?;

//         match join {
//             Ok(result) => Ok(result),
//             Err(e) => Err(e),
//         }
//     }

//     #[allow(dead_code)] // TODO: Remove if we will not use this method
//     pub async fn get_file_statuses(&self) -> joinerror::Result<HashMap<PathBuf, FileStatus>> {
//         let repo_handle_clone = self.repository.clone();
//         let join = tokio::task::spawn_blocking(move || {
//             let repo_handle_lock = repo_handle_clone.lock()?;
//             let repo_handle_ref = repo_handle_lock
//                 .as_ref()
//                 .ok_or_join_err::<()>("no repo handle")?;
//             repo_handle_ref.statuses()
//         })
//         .await?;

//         match join {
//             Ok(result) => Ok(result),
//             Err(e) => Err(e),
//         }
//     }

//     // FIXME: Maybe it doesn't make sense to have a separate method just to get the current branch name
//     // Although we don't need any comparison with remote branch just for getting the name
//     #[allow(dead_code)] // TODO: Remove if we will not use this method
//     pub async fn get_current_branch(&self) -> joinerror::Result<String> {
//         let repo_handle_clone = self.repository.clone();
//         let join = tokio::task::spawn_blocking(move || {
//             let repo_handle_lock = repo_handle_clone.lock()?;
//             let repo_handle_ref = repo_handle_lock
//                 .as_ref()
//                 .ok_or_join_err::<()>("no repo handle")?;
//             let current_branch = repo_handle_ref.current_branch()?;

//             Ok(current_branch)
//         })
//         .await?;

//         match join {
//             Ok(result) => Ok(result),
//             Err(e) => Err(e),
//         }
//     }

//     pub async fn get_current_branch_info(&self) -> joinerror::Result<BranchInfo> {
//         let repo_handle_clone = self.repository.clone();

//         // let join = tokio::task::spawn_blocking(move || {
//         //     let repo_handle_lock = repo_handle_clone.lock()?;
//         //     let repo_handle_ref = repo_handle_lock
//         //         .as_ref()
//         //         .ok_or_join_err::<()>("no repo handle")?;
//         //     // TODO: Support custom origin name? We assume it's `origin` now, which we use when we create a repo

//         //     let current_branch = repo_handle_ref.current_branch()?;

//         //     let mut output = BranchInfo {
//         //         name: current_branch.to_string(),
//         //         ahead: None,
//         //         behind: None,
//         //     };
//         //     // git fetch
//         //     if let Err(e) = repo_handle_ref.fetch(None) {
//         //         // This means that we cannot get the latest info about the upstream branch
//         //         // However, the operation can still succeed as long as a remote-tracking branch exists
//         //         // Just that the results might be outdated
//         //         // TODO: tell the frontend
//         //         println!("failed to fetch from the remote repo: {}", e.to_string())
//         //     }

//         //     // Compare local with remote state
//         //     // Even if we failed to compare with the remote branch, we can still return the current branch
//         //     match repo_handle_ref.graph_ahead_behind(&current_branch) {
//         //         Ok((ahead, behind)) => {
//         //             output.ahead = Some(ahead);
//         //             output.behind = Some(behind);
//         //         }
//         //         Err(e) => {
//         //             // TODO: tell the frontend
//         //             println!(
//         //                 "failed to compare local branch with remote branch: {}",
//         //                 e.to_string()
//         //             )
//         //         }
//         //     }

//         //     Ok(output)
//         // })
//         // .await?;

//         // match join {
//         //     Ok(result) => Ok(result),
//         //     Err(e) => Err(e),
//         // }

//         let repo_handle_lock = repo_handle_clone.lock()?;
//         let repo_handle_ref = repo_handle_lock
//             .as_ref()
//             .ok_or_join_err::<()>("no repo handle")?;
//         // TODO: Support custom origin name? We assume it's `origin` now, which we use when we create a repo

//         let current_branch = repo_handle_ref.current_branch()?;

//         let mut output = BranchInfo {
//             name: current_branch.to_string(),
//             ahead: None,
//             behind: None,
//         };

//         let (access_token, username) = match &self.client {
//             GitClient::GitHub { account, .. } => {
//                 (account.session().access_token().await?, account.username())
//             }
//             GitClient::GitLab { account, .. } => {
//                 (account.session().access_token().await?, account.username())
//             }
//         };

//         let mut cb = git2::RemoteCallbacks::new();
//         cb.credentials(move |_url, username_from_url, _allowed| {
//             // let rt = tokio::runtime::Handle::try_current();
//             // let fut = self.session_for_remote(ws, repo_root, remote_name);
//             // let (acc, tok) = match rt {
//             //     Ok(h) => h.block_on(fut),
//             //     Err(_) => tokio::runtime::Runtime::new().unwrap().block_on(fut),
//             // }
//             // .map_err(|e| git2::Error::from_str(&format!("auth error: {e}")))?;
//             // let user = username_from_url.unwrap_or(&acc.login);

//             git2::Cred::userpass_plaintext(username_from_url.unwrap_or(&username), &access_token)
//         });

//         // git fetch
//         if let Err(e) = repo_handle_ref.fetch(None, cb) {
//             // This means that we cannot get the latest info about the upstream branch
//             // However, the operation can still succeed as long as a remote-tracking branch exists
//             // Just that the results might be outdated
//             // TODO: tell the frontend
//             println!("failed to fetch from the remote repo: {}", e.to_string())
//         }

//         // Compare local with remote state
//         // Even if we failed to compare with the remote branch, we can still return the current branch
//         match repo_handle_ref.graph_ahead_behind(&current_branch) {
//             Ok((ahead, behind)) => {
//                 output.ahead = Some(ahead);
//                 output.behind = Some(behind);
//             }
//             Err(e) => {
//                 // TODO: tell the frontend
//                 println!(
//                     "failed to compare local branch with remote branch: {}",
//                     e.to_string()
//                 )
//             }
//         }

//         todo!()
//     }
// }
