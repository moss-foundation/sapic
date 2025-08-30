use moss_fs::{FileSystem, RemoveOptions};
use moss_git::repository::Repository;
use moss_git_hosting_provider::models::primitives::GitProviderType;
use moss_user::models::primitives::AccountId;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::git::GitClient;

pub trait CollectionVcs {
    fn owner(&self) -> AccountId;
    fn provider(&self) -> GitProviderType;
}

pub struct Vcs {
    repository: Arc<RwLock<Option<Repository>>>,
    client: GitClient,
}

impl Vcs {
    pub(crate) fn new(repository: Repository, client: GitClient) -> Self {
        Self {
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

impl CollectionVcs for Vcs {
    fn owner(&self) -> AccountId {
        self.client.owner()
    }

    fn provider(&self) -> GitProviderType {
        self.client.provider()
    }
}
