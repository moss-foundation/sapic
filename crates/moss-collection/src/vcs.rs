use async_trait::async_trait;
use moss_fs::{FileSystem, RemoveOptions};
use moss_git::{models::types::BranchInfo, repository::Repository, url::GitUrl};
use moss_git_hosting_provider::GitProviderKind;
use moss_user::models::primitives::AccountId;
use std::sync::Arc;
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
pub trait CollectionVcs: Send + Sync {
    async fn summary(&self) -> joinerror::Result<VcsSummary>;
    async fn contributors(&self) -> joinerror::Result<Vec<ContributorInfo>>;
    fn owner(&self) -> AccountId;
    fn provider(&self) -> GitProviderKind;
}

pub(crate) struct Vcs {
    url: GitUrl,
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
impl CollectionVcs for Vcs {
    async fn summary(&self) -> joinerror::Result<VcsSummary> {
        let repo = self.client.repository(&self.url).await?;

        let repo_lock = self.repository.read().await;
        let current_branch_name = repo_lock.as_ref().unwrap().current_branch()?;
        let (ahead, behind) = repo_lock
            .as_ref()
            .unwrap()
            .graph_ahead_behind(&current_branch_name)?;

        Ok(VcsSummary {
            kind: self.provider(),
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

    async fn contributors(&self) -> joinerror::Result<Vec<ContributorInfo>> {
        self.client.contributors(&self.url).await
    }
}
