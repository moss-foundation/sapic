pub mod api;
mod utils;
pub mod worktree;

use anyhow::{Context, Result};
use moss_fs::{FileSystem, RenameOptions};
use moss_storage::CollectionStorage;
use moss_storage::collection_storage::CollectionStorageImpl;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::OnceCell;
use worktree::Worktree;

#[derive(Clone, Debug)]
pub struct CollectionCache {
    pub name: String,
    pub order: Option<usize>,
}

pub struct Collection {
    fs: Arc<dyn FileSystem>,
    worktree: OnceCell<Arc<Worktree>>,
    abs_path: PathBuf,
    collection_storage: Arc<dyn CollectionStorage>,
    next_entry_id: Arc<AtomicUsize>,
}

impl Collection {
    pub fn new(
        path: PathBuf,
        fs: Arc<dyn FileSystem>,
        next_entry_id: Arc<AtomicUsize>,
    ) -> Result<Self> {
        debug_assert!(path.is_absolute());

        let state_db_manager_impl = CollectionStorageImpl::new(&path).context(format!(
            "Failed to open the collection {} state database",
            path.display()
        ))?;

        Ok(Self {
            fs: Arc::clone(&fs),
            abs_path: path,
            worktree: OnceCell::new(),
            collection_storage: Arc::new(state_db_manager_impl),
            next_entry_id,
        })
    }

    pub async fn worktree(&self) -> Result<&Arc<Worktree>> {
        self.worktree
            .get_or_try_init(|| async move {
                let worktree = Worktree::new(
                    self.fs.clone(),
                    Arc::from(self.abs_path.clone()),
                    self.next_entry_id.clone(),
                );

                Ok(Arc::new(worktree))
            })
            .await
    }

    pub fn path(&self) -> &PathBuf {
        &self.abs_path
    }

    pub async fn reset(&mut self, new_path: Arc<Path>) -> Result<()> {
        debug_assert!(new_path.is_absolute());

        let old_path = std::mem::replace(&mut self.abs_path, new_path.to_path_buf());
        let fs_clone = self.fs.clone();
        let new_path_clone = new_path.clone();

        let after_drop = Box::pin(async move {
            fs_clone
                .rename(&old_path, &new_path_clone, RenameOptions::default())
                .await?;

            Ok(())
        });

        self.collection_storage.reset(&new_path, after_drop).await?;

        Ok(())
    }
}
