use std::{
    collections::{BTreeSet, HashMap, HashSet},
    path::{Path, PathBuf},
};

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
        let entry = worktree
            .entry(input.id)
            .ok_or(OperationError::NotFound(input.id.to_string()))?;

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

        let r = filter_by_expanded_ancestors(&entry.path, expanded_paths);

        worktree.load_many(r.into_iter()).await?;

        // let changes = worktree.restore(&input.path, expanded_paths).await?;

        // let changes = worktree.load_entry(&input.path, input.depth).await?;

        // worktree.walk(id, resolver, sender).await?;

        // Ok(ExpandEntryOutput { changes })

        todo!()
    }
}

pub fn filter_by_expanded_ancestors<'a>(
    boundary: &Path,
    expanded_paths: impl IntoIterator<Item = &'a str>,
) -> Vec<PathBuf> {
    let path_strings: Vec<&str> = expanded_paths.into_iter().map(|s| s).collect();
    let path_set: HashSet<PathBuf> = path_strings.iter().map(PathBuf::from).collect();

    let mut result = vec![boundary.to_path_buf()];

    'path_loop: for path_str in &path_strings {
        let p = PathBuf::from(path_str);

        if !boundary.as_os_str().is_empty() && !p.starts_with(boundary) {
            continue 'path_loop; // Boundary is not empty
        }

        let mut current = p.clone();
        while let Some(parent) = current.parent() {
            if parent == boundary {
                break; // Reached boundary
            }

            if parent.as_os_str().is_empty() {
                if boundary.as_os_str().is_empty() {
                    break;
                } else {
                    continue 'path_loop; // Boundary is not empty, root is above boundary
                }
            }

            if !path_set.contains(parent) {
                continue 'path_loop; // Parent is not in the original set
            }
            current = parent.to_path_buf();
        }

        result.push(p);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_without_boundary_root() {
        let input = vec!["a", "a/b", "a/b/c/d"];
        let boundary = Path::new("");
        let result = filter_by_expanded_ancestors(&boundary, input.clone());
        assert_eq!(
            result,
            vec![PathBuf::from(""), PathBuf::from("a"), PathBuf::from("a/b")]
        );
    }

    #[test]
    fn test_filter_with_boundary() {
        let input = vec!["a", "a/b", "a/b/c/d"];
        let boundary = Path::new("a/b");
        let result = filter_by_expanded_ancestors(&boundary, input);

        assert_eq!(result, vec![PathBuf::from("a/b")]);
    }

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
