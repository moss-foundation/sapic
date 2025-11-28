use async_trait::async_trait;
use git2::{RemoteCallbacks, Signature};
use joinerror::OptionExt;
use moss_app_delegate::{AppDelegate, broadcast::ToLocation};
use moss_applib::{AppRuntime, errors::Internal};
use moss_fs::{FileSystem, RemoveOptions};
use moss_git::{
    errors::{Conflicts, DirtyWorktree},
    models::{primitives::FileStatus, types::BranchInfo},
    repository::Repository,
    url::GitUrl,
};
use sapic_base::user::types::primitives::AccountId;
use sapic_core::context::AnyAsyncContext;
use sapic_system::ports::GitProviderKind;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::git::{ContributorInfo, GitClient, OwnerInfo};

pub struct VcsSummary {
    pub kind: GitProviderKind,
    pub branch: BranchInfo,
    pub url: String,
    pub updated_at: Option<String>,
    pub owner: Option<OwnerInfo>,
}

#[async_trait]
pub trait ProjectVcs<R: AppRuntime>: Send + Sync {
    async fn summary(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<VcsSummary>;
    async fn contributors(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<ContributorInfo>>;
    async fn statuses(&self) -> joinerror::Result<HashMap<PathBuf, FileStatus>>;
    fn owner(&self) -> AccountId;
    fn provider(&self) -> GitProviderKind;

    async fn stage_and_commit(&self, paths: Vec<PathBuf>, message: &str) -> joinerror::Result<()>;
    async fn push(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<()>;
    async fn pull(
        &self,
        ctx: &dyn AnyAsyncContext,
        app_delegate: &AppDelegate<R>,
    ) -> joinerror::Result<()>;
    async fn fetch(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<()>;
    async fn discard_changes(&self, paths: Vec<PathBuf>) -> joinerror::Result<()>;
}

pub(crate) struct Vcs {
    url: GitUrl,
    // An Option is used here to allow dropping it separately when cleaning up
    repository: Arc<RwLock<Option<Repository>>>,
    client: GitClient,
}

impl Vcs {
    pub(crate) fn new(url: GitUrl, repository: Repository, client: GitClient) -> Self {
        Self {
            url,
            repository: Arc::new(RwLock::new(Some(repository))),
            client,
        }
    }

    // Sometimes objects might be set as readonly, preventing them from being deleted
    // we will need to recursively set all files in .git/objects as writable
    pub(crate) async fn dispose(&self, fs: Arc<dyn FileSystem>) -> joinerror::Result<()> {
        let repo_handle = self.repository.write().await.take();
        if repo_handle.is_none() {
            return Ok(());
        }
        let repo_handle = repo_handle.unwrap();
        let path = repo_handle.path().to_path_buf();
        drop(repo_handle);

        let mut folders = vec![path.join("objects")];

        while let Some(folder) = folders.pop() {
            let mut read_dir = fs.read_dir(&folder).await?;
            while let Some(entry) = read_dir.next_entry().await? {
                if entry.file_type().await?.is_dir() {
                    folders.push(entry.path());
                }
                let mut perms = entry.metadata().await?.permissions();
                perms.set_readonly(false);
                tokio::fs::set_permissions(&entry.path(), perms).await?;
            }
        }

        fs.remove_dir(
            &path,
            RemoveOptions {
                recursive: true,
                ignore_if_not_exists: true,
            },
        )
        .await?;

        Ok(())
    }
}

#[async_trait]
impl<R: AppRuntime> ProjectVcs<R> for Vcs {
    async fn summary(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<VcsSummary> {
        let repo = self.client.repository(ctx, &self.url).await?;

        let repo_lock = self.repository.read().await;
        let current_branch_name = repo_lock
            .as_ref()
            .ok_or_join_err::<Internal>("repository handle is dropped")?
            .current_branch()?;
        let (ahead, behind) = repo_lock
            .as_ref()
            .unwrap()
            .graph_ahead_behind(&current_branch_name)?;

        Ok(VcsSummary {
            kind: self.client.kind(),
            branch: BranchInfo {
                name: current_branch_name,
                ahead: Some(ahead),
                behind: Some(behind),
            },
            url: self.url.to_string()?,
            updated_at: Some(repo.updated_at),
            owner: Some(repo.owner),
        })
    }

    fn owner(&self) -> AccountId {
        self.client.account_id()
    }

    fn provider(&self) -> GitProviderKind {
        self.client.kind()
    }

    async fn contributors(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<ContributorInfo>> {
        self.client.contributors(ctx, &self.url).await
    }

    async fn statuses(&self) -> joinerror::Result<HashMap<PathBuf, FileStatus>> {
        let repo_lock = self.repository.read().await;
        let repo_ref = repo_lock
            .as_ref()
            .ok_or_join_err::<Internal>("repository handle is dropped")?;

        repo_ref.statuses()
    }

    async fn stage_and_commit(&self, paths: Vec<PathBuf>, message: &str) -> joinerror::Result<()> {
        let repo_lock = self.repository.read().await;
        let repo_ref = repo_lock
            .as_ref()
            .ok_or_join_err::<Internal>("repository handle is dropped")?;

        repo_ref.stage_paths(paths)?;
        let username = self.client.username();
        repo_ref.commit(
            message,
            Signature::now(
                &username,
                // FIXME: use actual commit email
                &format!("{}@git.noreply.com", &username),
            )?,
        )?;

        Ok(())
    }

    // Pushing currently checked-out branch to the configured refspec+remote
    async fn push(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<()> {
        let repo_lock = self.repository.read().await;
        let repo_ref = repo_lock
            .as_ref()
            .ok_or_join_err::<Internal>("repository handle is dropped")?;

        let username = self.client.username();
        let token = self.client.session().token(ctx).await?;
        let mut cb = RemoteCallbacks::new();
        cb.credentials(move |_url, username_from_url, _allowed| {
            git2::Cred::userpass_plaintext(&username_from_url.unwrap_or(&username), &token)
        });

        repo_ref.push(None, None, None, false, cb)?;

        Ok(())
    }

    async fn pull(
        &self,
        ctx: &dyn AnyAsyncContext,
        app_delegate: &AppDelegate<R>,
    ) -> joinerror::Result<()> {
        let repo_lock = self.repository.write().await;
        let repo_ref = repo_lock
            .as_ref()
            .ok_or_join_err::<Internal>("repository handle is dropped")?;

        let username = self.client.username();
        let token = self.client.session().token(ctx).await?;
        let mut cb = RemoteCallbacks::new();
        cb.credentials(move |_url, username_from_url, _allowed| {
            git2::Cred::userpass_plaintext(&username_from_url.unwrap_or(&username), &token)
        });

        let result = repo_ref.pull(None, cb);
        if result.is_ok() {
            return Ok(());
        }

        let e = result.unwrap_err();
        if e.is::<DirtyWorktree>() {
            // Prompt the user to stash before pulling
            let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                activity_id: "pull_dirty_worktree",
                title: "Failed to pull due to dirty worktree".to_string(),
                detail: Some(
                    "Please stash your changes before pulling to avoid loss of local changes"
                        .to_string(),
                ),
            });
        } else if e.is::<Conflicts>() {
            // Right now our app cannot handle conflict resolution
            // Thus we have to reject merge/pull that result in conflicts
            let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                activity_id: "pull_conflicts",
                title: "Failed to pull due to dirty worktree".to_string(),
                detail: Some(
                    "Please manually pull and resolve conflicts, as the functionality is WIP"
                        .to_string(),
                ),
            });
        }
        Err(e)
    }

    async fn fetch(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<()> {
        let repo_lock = self.repository.read().await;
        let repo_ref = repo_lock
            .as_ref()
            .ok_or_join_err::<Internal>("repository handle is dropped")?;

        let username = self.client.username();
        let token = self.client.session().token(ctx).await?;
        let mut cb = RemoteCallbacks::new();
        cb.credentials(move |_url, username_from_url, _allowed| {
            git2::Cred::userpass_plaintext(&username_from_url.unwrap_or(&username), &token)
        });

        repo_ref.fetch(None, cb)?;

        Ok(())
    }

    async fn discard_changes(&self, paths: Vec<PathBuf>) -> joinerror::Result<()> {
        let repo_lock = self.repository.read().await;
        let repo_ref = repo_lock
            .as_ref()
            .ok_or_join_err::<Internal>("repository handle is dropped")?;

        repo_ref.discard_changes(paths)?;

        Ok(())
    }
}
