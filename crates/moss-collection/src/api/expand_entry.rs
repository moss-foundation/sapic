use std::collections::{HashMap, HashSet};

use moss_common::api::{OperationError, OperationResult};
use moss_storage::storage::operations::GetItem;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    Collection,
    models::{
        operations::{ExpandEntryInput, ExpandEntryOutput},
        primitives::WorktreeChange,
    },
    storage::{WorktreeNodeStateEntity, segments::SEGKEY_FOLDERS_STATE},
};

impl Collection {
    pub async fn expand_entry(
        &mut self,
        channel: TauriChannel<ExpandEntryOutput>,
        input: ExpandEntryInput,
    ) -> OperationResult<ExpandEntryOutput> {
        let folders_state = GetItem::get(
            self.storage().mixed_store().as_ref(),
            SEGKEY_FOLDERS_STATE.to_segkey_buf(),
        )?
        .deserialize::<HashMap<String, WorktreeNodeStateEntity>>()?;

        let worktree = self.worktree_mut().await?;

        let expanded_paths = folders_state
            .iter()
            .filter_map(|(path, state)| {
                if state.expanded {
                    Some(path.as_str())
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        // worktree.expand_entry(&input.path, expanded_paths).await?;

        // let changes = worktree.load_entry(&input.path, input.depth).await?;

        // worktree.walk(id, resolver, sender).await?;

        // Ok(ExpandEntryOutput { changes })

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_expand_entry() {
        // (...) - expanded node
        // (*) - root node
        //
        //           "*"
        //         /     \
        //       (a)      b
        //       / \     / \
        //      c   d  (e)   f
        //     /       / \
        //   (g)     (i)   j
        //   / \     / \
        //  k   l  (m)  n
        //         / \
        //        o   p

        let test_data = HashMap::from([
            (
                "",
                WorktreeNodeStateEntity {
                    expanded: true,
                    order: None,
                },
            ),
            // ---
            // a
            // ---
            (
                "a",
                WorktreeNodeStateEntity {
                    expanded: true,
                    order: None,
                },
            ),
            (
                "a/c",
                WorktreeNodeStateEntity {
                    expanded: false,
                    order: None,
                },
            ),
            (
                "a/d",
                WorktreeNodeStateEntity {
                    expanded: false,
                    order: None,
                },
            ),
            // ---
            // a/c
            // ---
            (
                "a/c/g",
                WorktreeNodeStateEntity {
                    expanded: false,
                    order: None,
                },
            ),
            // ---
            // a/c/g
            // ---
            (
                "a/c/g/k",
                WorktreeNodeStateEntity {
                    expanded: false,
                    order: None,
                },
            ),
            (
                "a/c/g/l",
                WorktreeNodeStateEntity {
                    expanded: false,
                    order: None,
                },
            ),
            // ---
            // b
            // ---
            (
                "b",
                WorktreeNodeStateEntity {
                    expanded: false,
                    order: None,
                },
            ),
            (
                "b/e",
                WorktreeNodeStateEntity {
                    expanded: false,
                    order: None,
                },
            ),
            (
                "b/f",
                WorktreeNodeStateEntity {
                    expanded: false,
                    order: None,
                },
            ),
            // ---
            // b/e
            // ---
            (
                "b/e/i",
                WorktreeNodeStateEntity {
                    expanded: false,
                    order: None,
                },
            ),
            (
                "b/e/j",
                WorktreeNodeStateEntity {
                    expanded: false,
                    order: None,
                },
            ),
        ]);

        // let mut collection = Collection::new().await.unwrap();

        // collection.storage().mixed_store().put(SEGKEY_FOLDERS_STATE.to_segkey_buf(), test_data).await.unwrap();
    }
}
