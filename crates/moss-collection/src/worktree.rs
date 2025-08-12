pub mod entry;

use anyhow::anyhow;
use derive_more::{Deref, DerefMut};
use joinerror::OptionExt;
use json_patch::{PatchOperation, ReplaceOperation, jsonptr::PointerBuf};
use moss_applib::AppRuntime;
use moss_common::{continue_if_err, continue_if_none};
use moss_db::primitives::AnyValue;
use moss_edit::json::EditOptions;
use moss_fs::{CreateOptions, FileSystem, RemoveOptions, desanitize_path, utils::SanitizedPath};
use moss_hcl::HclResultExt;
use moss_storage::primitives::segkey::SegKeyBuf;
use moss_text::sanitized::{desanitize, sanitize};
use rustc_hash::FxHashMap;
use serde_json::Value as JsonValue;
use std::{
    cell::LazyCell,
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    fs,
    sync::{RwLock, mpsc, watch},
};

use crate::{
    constants::{self, COLLECTION_ROOT_PATH, DIR_CONFIG_FILENAME},
    models::{
        primitives::{
            EntryClass, EntryId, EntryKind, EntryProtocol, HeaderId, PathParamId, QueryParamId,
        },
        types::http::{
            AddHeaderParams, AddPathParamParams, AddQueryParamParams, UpdateHeaderParams,
            UpdatePathParamParams, UpdateQueryParamParams,
        },
    },
    storage::segments,
    worktree::entry::{edit::EntryEditing, model::EntryModel},
};

use crate::{
    errors::{ErrorAlreadyExists, ErrorInvalidInput, ErrorInvalidKind, ErrorIo, ErrorNotFound},
    services::storage_service::StorageService,
};

const CLASS_TO_DIR_NAME: LazyCell<FxHashMap<EntryClass, &str>> = LazyCell::new(|| {
    [
        (EntryClass::Request, "requests"),
        (EntryClass::Endpoint, "endpoints"),
        (EntryClass::Component, "components"),
        (EntryClass::Schema, "schemas"),
    ]
    .into_iter()
    .collect::<FxHashMap<_, _>>()
});

#[derive(Debug)]
struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    scan_queue: mpsc::UnboundedSender<ScanJob>,
}

// pub enum EntryConfiguration {
//     Item(RawItemConfiguration),
//     Dir(RawDirConfiguration),
// }

// impl EntryConfiguration {
//     pub fn as_item(&self) -> Option<&RawItemConfiguration> {
//         match self {
//             EntryConfiguration::Item(conf) => Some(conf),
//             EntryConfiguration::Dir(_) => None,
//         }
//     }

//     pub fn as_dir(&self) -> Option<&RawDirConfiguration> {
//         match self {
//             EntryConfiguration::Item(_) => None,
//             EntryConfiguration::Dir(conf) => Some(conf),
//         }
//     }

//     pub fn classification(&self) -> EntryClass {
//         match self {
//             EntryConfiguration::Item(conf) => match conf {
//                 RawItemConfiguration::Request(_) => EntryClass::Request,
//                 RawItemConfiguration::Endpoint(_) => EntryClass::Endpoint,
//                 RawItemConfiguration::Component(_) => EntryClass::Component,
//                 RawItemConfiguration::Schema(_) => EntryClass::Schema,
//             },
//             EntryConfiguration::Dir(conf) => conf.classification(),
//         }
//     }

//     pub fn protocol(&self) -> Option<EntryProtocol> {
//         match self {
//             EntryConfiguration::Item(conf) => match conf {
//                 RawItemConfiguration::Request(conf) => conf.url.protocol(),
//                 RawItemConfiguration::Endpoint(conf) => conf.url.protocol(),
//                 RawItemConfiguration::Component(_) => None,
//                 RawItemConfiguration::Schema(_) => None,
//             },
//             EntryConfiguration::Dir(_) => None,
//         }
//     }

//     pub fn kind(&self) -> EntryKind {
//         match self {
//             EntryConfiguration::Item(_) => EntryKind::Item,
//             EntryConfiguration::Dir(_) => EntryKind::Dir,
//         }
//     }
// }

pub(crate) struct ModifyParams {
    pub name: Option<String>,
    pub protocol: Option<EntryProtocol>,
    pub expanded: Option<bool>,
    pub order: Option<isize>,
    pub path: Option<PathBuf>,

    pub query_params_to_add: Vec<AddQueryParamParams>,
    pub query_params_to_update: Vec<UpdateQueryParamParams>,
    pub query_params_to_remove: Vec<QueryParamId>,
    //
    //TODO: Add
    // pub path_params_to_add: Vec<AddPathParamParams>,
    // pub path_params_to_update: Vec<UpdatePathParamParams>,
    // pub path_params_to_remove: Vec<PathParamId>,

    // pub headers_to_add: Vec<AddHeaderParams>,
    // pub headers_to_update: Vec<UpdateHeaderParams>,
    // pub headers_to_remove: Vec<HeaderParamId>,
}

// #[derive(Deref, DerefMut)]
// pub struct EntryItemMut<'a> {
//     path: &'a mut Arc<Path>,

//     #[deref]
//     #[deref_mut]
//     configuration: &'a mut RawItemConfiguration,
// }

// #[derive(Deref, DerefMut)]
// pub struct EntryDirMut<'a> {
//     path: &'a mut Arc<Path>,

//     #[deref]
//     #[deref_mut]
//     configuration: &'a mut RawDirConfiguration,
// }

#[derive(Deref, DerefMut)]
struct Entry {
    id: EntryId,
    path_rx: watch::Receiver<Arc<Path>>,

    #[deref]
    #[deref_mut]
    edit: EntryEditing,
    class: EntryClass,
    protocol: Option<EntryProtocol>,
    // configuration: EntryConfiguration,
}

impl Entry {}

// impl Entry {
//     pub fn as_item_mut(&mut self) -> Option<EntryItemMut<'_>> {
//         match &mut self.configuration {
//             EntryConfiguration::Item(configuration) => Some(EntryItemMut {
//                 path: &mut self.path,
//                 configuration,
//             }),
//             _ => None,
//         }
//     }

//     pub fn as_dir_mut(&mut self) -> Option<EntryDirMut<'_>> {
//         match &mut self.configuration {
//             EntryConfiguration::Dir(configuration) => Some(EntryDirMut {
//                 path: &mut self.path,
//                 configuration,
//             }),
//             _ => None,
//         }
//     }
// }

#[derive(Debug)]
pub struct EntryDescription {
    pub id: EntryId,
    pub name: String,
    pub path: Arc<Path>,
    pub class: EntryClass,
    pub kind: EntryKind,
    pub protocol: Option<EntryProtocol>,
    pub order: Option<isize>,
    pub expanded: bool,
}

#[derive(Default)]
struct WorktreeState {
    entries: HashMap<EntryId, Entry>,
    expanded_entries: HashSet<EntryId>,
}

pub(crate) struct Worktree<R: AppRuntime> {
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    storage: Arc<StorageService<R>>,
    state: Arc<RwLock<WorktreeState>>,
}

impl<R: AppRuntime> Worktree<R> {
    pub fn absolutize(&self, path: &Path) -> joinerror::Result<PathBuf> {
        debug_assert!(path.is_relative());

        if path
            .components()
            .any(|c| c == std::path::Component::ParentDir)
        {
            return Err(joinerror::Error::new::<ErrorInvalidInput>(format!(
                "Path cannot contain '..' components: {}",
                path.display()
            )));
        }

        if path.file_name().is_some() {
            Ok(self.abs_path.join(path))
        } else {
            Ok(self.abs_path.to_path_buf())
        }
    }

    pub async fn remove_entry(&self, ctx: &R::AsyncContext, id: &EntryId) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;
        let entry = state_lock
            .entries
            .remove(&id)
            .ok_or_join_err_with::<ErrorNotFound>(|| format!("entry {} not found", id))?;

        let abs_path = self.absolutize(&entry.path_rx.borrow())?;
        if !abs_path.exists() {
            return Err(joinerror::Error::new::<ErrorNotFound>(format!(
                "Entry not found: {}",
                abs_path.display()
            )));
        }

        self.fs
            .remove_dir(
                &abs_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        state_lock.expanded_entries.remove(&id);
        self.storage
            .put_expanded_entries(
                ctx,
                state_lock
                    .expanded_entries
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>(),
            )
            .await?;

        Ok(())
    }

    pub async fn scan(
        &self,
        _ctx: &R::AsyncContext, // TODO: use ctx ctx.done() to cancel the scan if needed
        path: &Path,
        expanded_entries: Arc<HashSet<EntryId>>,
        all_entry_keys: Arc<HashMap<SegKeyBuf, AnyValue>>,
        sender: mpsc::UnboundedSender<EntryDescription>,
    ) -> joinerror::Result<()> {
        debug_assert!(path.is_relative());

        let path: Arc<Path> = path.into();
        let abs_path = self.absolutize(&path)?;

        let (job_tx, mut job_rx) = mpsc::unbounded_channel();

        let initial_job = ScanJob {
            abs_path: abs_path.into(),
            path: Arc::clone(&path),
            scan_queue: job_tx.clone(),
        };
        job_tx.send(initial_job).unwrap();

        drop(job_tx);

        let mut handles = Vec::new();
        while let Some(job) = job_rx.recv().await {
            let sender = sender.clone();
            let fs = self.fs.clone();
            let state = self.state.clone();
            let expanded_entries = expanded_entries.clone();
            let all_entry_keys = all_entry_keys.clone();

            let handle = tokio::spawn(async move {
                let mut new_jobs = Vec::new();

                // // let dir_name = job
                // //     .path
                // //     .file_name()
                // //     .map(|n| n.to_string_lossy().to_string())
                // //     .unwrap_or_else(|| job.path.to_string_lossy().to_string());

                match process_entry(
                    job.path.clone(),
                    &all_entry_keys,
                    &expanded_entries,
                    &fs,
                    &job.abs_path,
                )
                .await
                {
                    Ok(Some((entry, desc))) => {
                        // let expanded = expanded_entries.contains(&entry.id);
                        // let desc = EntryDescription {
                        //     id: entry.id.clone(),
                        //     name: desanitize(&dir_name),
                        //     path: entry.path.clone(),
                        //     class: entry.classification(),
                        //     kind: entry.kind(),
                        //     protocol: entry.configuration.protocol(),
                        //     expanded,
                        //     order: all_entry_keys
                        //         .get(&segments::segkey_entry_order(&entry.id))
                        //         .and_then(|o| o.deserialize().ok()),
                        // };

                        if desc.expanded {
                            state
                                .write()
                                .await
                                .expanded_entries
                                .insert(entry.id.clone());
                        }

                        let _ = sender.send(desc);
                        state.write().await.entries.insert(entry.id.clone(), entry);
                    }
                    Ok(None) => {
                        // TODO: log error
                        return;
                    }
                    Err(_err) => {
                        eprintln!("Error processing dir {}: {}", job.path.display(), _err);
                        // TODO: log error
                        return;
                    }
                }

                let mut read_dir = match fs::read_dir(&job.abs_path).await {
                    Ok(dir) => dir,
                    Err(_) => return,
                };

                let mut child_paths = Vec::new();
                while let Ok(Some(dir_entry)) = read_dir.next_entry().await {
                    child_paths.push(dir_entry);
                }

                for child_entry in child_paths {
                    let child_file_type = continue_if_err!(child_entry.file_type().await);
                    let child_abs_path: Arc<Path> = child_entry.path().into();
                    let child_name = continue_if_none!(child_abs_path.file_name())
                        .to_string_lossy()
                        .to_string();
                    let child_path: Arc<Path> = job.path.join(&child_name).into();

                    let maybe_entry = if child_file_type.is_dir() {
                        continue_if_err!(
                            process_entry(
                                child_path.clone(),
                                &all_entry_keys,
                                &expanded_entries,
                                &fs,
                                &child_abs_path
                            )
                            .await
                        )
                    } else {
                        continue_if_err!(
                            process_file(&child_name, &child_path, &fs, &child_abs_path).await
                        )
                    };

                    let (entry, desc) = continue_if_none!(maybe_entry, || {
                        // TODO: Probably should log here since we should not be able to get here
                    });

                    // INFO: Something here doesn't feel quite right—maybe we can improve it once we have the UI
                    if child_file_type.is_dir() {
                        new_jobs.push(ScanJob {
                            abs_path: Arc::clone(&child_abs_path),
                            path: child_path,
                            scan_queue: job.scan_queue.clone(),
                        });
                    } else {
                        // let desc = EntryDescription {
                        //     id: entry.id.clone(),
                        //     name: desanitize(&child_name),
                        //     path: entry.path.clone(),
                        //     class: entry.classification(),
                        //     kind: entry.kind(),
                        //     protocol: entry.configuration.protocol(),
                        //     expanded: expanded_entries.contains(&entry.id),
                        //     order: all_entry_keys
                        //         .get(&segments::segkey_entry_order(&entry.id))
                        //         .and_then(|o| o.deserialize().ok()),
                        // };

                        continue_if_err!(sender.send(desc), |_err| {
                            eprintln!("Error sending entry: {}", _err);
                            // TODO: log error
                        });
                    }

                    state.write().await.entries.insert(entry.id.clone(), entry);
                }

                for new_job in new_jobs {
                    continue_if_err!(job.scan_queue.send(new_job), |_err| {
                        eprintln!("Error sending new job: {}", _err);
                        // TODO: log error
                    });
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Err(_err) = handle.await {
                // TODO: log error
            }
        }

        Ok(())
    }

    pub async fn create_item_entry(
        &self,
        ctx: &R::AsyncContext,
        name: &str,
        path: &Path,
        model: EntryModel,
        order: isize,
        expanded: bool,
    ) -> joinerror::Result<()> {
        debug_assert!(path.is_relative());

        if !is_parent_dir_entry(self.abs_path.as_ref(), path) {
            return Err(joinerror::Error::new::<ErrorInvalidInput>(format!(
                "Cannot create entry inside Item entry {}",
                path.to_string_lossy().to_string()
            )));
        }

        let sanitized_path: SanitizedPath = moss_fs::utils::sanitize_path(path, None)?
            .join(sanitize(name))
            .into();

        let content = hcl::to_string(&model)
            .join_err::<()>("failed to serialize configuration into hcl string")?;
        self.create_entry(&sanitized_path, false, &content.as_bytes())
            .await?;

        let mut state_lock = self.state.write().await;

        let id = model.id().clone();
        let (path_tx, path_rx) = watch::channel(sanitized_path.to_path_buf().into());

        state_lock.entries.insert(
            id.clone(),
            Entry {
                id: id.clone(),
                // path: sanitized_path.to_path_buf().into(),
                // configuration: EntryConfiguration::Item(configuration),
                path_rx,
                edit: EntryEditing::new(self.fs.clone(), path_tx),
                class: model.metadata.class.clone(),
                protocol: model.protocol(),
            },
        );

        {
            let mut txn = self.storage.begin_write(ctx).await?;

            self.storage
                .put_entry_order_txn(ctx, &mut txn, &id, order)
                .await?;

            if expanded {
                state_lock.expanded_entries.insert(id);

                self.storage
                    .put_expanded_entries_txn(
                        ctx,
                        &mut txn,
                        state_lock
                            .expanded_entries
                            .iter()
                            .cloned()
                            .collect::<Vec<_>>(),
                    )
                    .await?;
            }

            txn.commit()?;
        }

        Ok(())
    }

    pub async fn create_dir_entry(
        &self,
        ctx: &R::AsyncContext,
        name: &str,
        path: &Path,
        model: EntryModel,
        order: isize,
        expanded: bool,
    ) -> joinerror::Result<()> {
        debug_assert!(path.is_relative());

        if !is_parent_dir_entry(self.abs_path.as_ref(), path) {
            return Err(joinerror::Error::new::<ErrorInvalidInput>(format!(
                "Cannot create entry inside Item entry {}",
                path.to_string_lossy().to_string()
            )));
        }

        let sanitized_path: SanitizedPath = moss_fs::utils::sanitize_path(path, None)?
            .join(sanitize(name))
            .into();

        let content = hcl::to_string(&model)
            .join_err::<()>("failed to serialize configuration into hcl string")?;
        self.create_entry(&sanitized_path, true, &content.as_bytes())
            .await?;

        let mut state_lock = self.state.write().await;

        let id = model.id().clone();
        let (path_tx, path_rx) = watch::channel(sanitized_path.to_path_buf().into());
        state_lock.entries.insert(
            id.clone(),
            Entry {
                id: id.clone(),
                // path: sanitized_path.to_path_buf().into(),
                // configuration: EntryConfiguration::Dir(configuration),
                path_rx,
                edit: EntryEditing::new(self.fs.clone(), path_tx),
                class: model.metadata.class.clone(),
                protocol: None,
            },
        );

        {
            let mut txn = self.storage.begin_write(ctx).await?;
            self.storage
                .put_entry_order_txn(ctx, &mut txn, &id, order)
                .await?;

            if expanded {
                state_lock.expanded_entries.insert(id);

                self.storage
                    .put_expanded_entries_txn(
                        ctx,
                        &mut txn,
                        state_lock
                            .expanded_entries
                            .iter()
                            .cloned()
                            .collect::<Vec<_>>(),
                    )
                    .await?;
            }

            txn.commit()?;
        }

        Ok(())
    }

    pub async fn update_dir_entry(
        &self,
        ctx: &R::AsyncContext,
        id: &EntryId,
        params: ModifyParams,
    ) -> joinerror::Result<Arc<Path>> {
        let mut state_lock = self.state.write().await;
        let entry = state_lock
            .entries
            .get_mut(&id)
            .ok_or_join_err_with::<ErrorNotFound>(|| format!("entry {} not found", id))?;
        // .as_dir_mut()
        // .ok_or_join_err_with::<ErrorInvalidKind>(|| {
        //     format!("entry {} is not a directory", id)
        // })?;

        if let Some(new_parent) = params.path {
            if !new_parent.starts_with(CLASS_TO_DIR_NAME.get(&entry.class).unwrap()) {
                return Err(joinerror::Error::new::<ErrorInvalidInput>(
                    "cannot move entry to a different classification folder",
                ));
            }

            // We can only move entries into a directory entry
            // Check if the destination path has dir config file
            let dest_entry_config = self.abs_path.join(&new_parent).join(DIR_CONFIG_FILENAME);
            if !dest_entry_config.exists() {
                return Err(joinerror::Error::new::<ErrorInvalidInput>(
                    "cannot move entries into a non-directory entry",
                ));
            }

            // let old_path = entry.path.clone();
            // let new_path = update_path_parent(entry.path.as_ref(), &new_parent)?;
            // path = new_path.clone();

            // self.rename_entry(&old_path, &new_path).await?;
            // *entry.path = new_path.into();

            let old_path = entry.path_rx.borrow().clone();
            let new_path = update_path_parent(&old_path, &new_parent)?;

            entry
                .edit
                .rename(&self.abs_path, &old_path, &new_path)
                .await?;
        }

        if let Some(name) = params.name {
            // let old_path = entry.path.clone();
            // let new_path = rename_path(entry.path.as_ref(), &name);
            // path = new_path.clone();

            // self.rename_entry(&old_path, &new_path).await?;
            // *entry.path = new_path.into();

            let old_path = entry.path_rx.borrow().clone();
            let new_path = rename_path(&old_path, &name);

            entry
                .edit
                .rename(&self.abs_path, &old_path, &new_path)
                .await?;
        }

        let path = entry.path_rx.borrow().clone();
        drop(state_lock);

        // let configuration = entry.configuration.clone();
        let is_db_update_needed = params.order.is_some() || params.expanded.is_some();
        if !is_db_update_needed {
            return Ok(path);
        }

        let mut txn = self.storage.begin_write(ctx).await?;

        if let Some(order) = params.order {
            self.storage
                .put_entry_order_txn(ctx, &mut txn, id, order)
                .await?;
        }

        let mut state_lock = self.state.write().await;
        if let Some(expanded) = params.expanded {
            if expanded {
                state_lock.expanded_entries.insert(id.to_owned());
            } else {
                state_lock.expanded_entries.remove(id);
            }

            self.storage
                .put_expanded_entries_txn(
                    ctx,
                    &mut txn,
                    state_lock
                        .expanded_entries
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                )
                .await?;
        }

        txn.commit()?;

        Ok(path)
    }

    pub async fn update_item_entry(
        &self,
        ctx: &R::AsyncContext,
        id: &EntryId,
        params: ModifyParams,
    ) -> joinerror::Result<Arc<Path>> {
        let mut state_lock = self.state.write().await;
        let entry = state_lock
            .entries
            .get_mut(&id)
            .ok_or_join_err_with::<ErrorNotFound>(|| format!("entry {} not found", id))?;
        // .as_item_mut()
        // .ok_or_join_err_with::<ErrorInvalidKind>(|| format!("entry {} is not a item", id))?;

        // let mut path = entry.path.clone().to_path_buf();

        if let Some(new_parent) = &params.path {
            if !new_parent.starts_with(CLASS_TO_DIR_NAME.get(&entry.class).unwrap()) {
                return Err(joinerror::Error::new::<ErrorInvalidInput>(
                    "cannot move entry to a different classification folder",
                ));
            }

            // We can only move entries into a directory entry
            // Check if the destination path has dir config file
            let dest_entry_config = self.abs_path.join(&new_parent).join(DIR_CONFIG_FILENAME);
            if !dest_entry_config.exists() {
                return Err(joinerror::Error::new::<ErrorInvalidInput>(
                    "cannot move entries into a non-directory entry",
                ));
            }

            // let old_path = entry.path.clone();
            // let new_path = update_path_parent(entry.path.as_ref(), &new_parent)?;
            // path = new_path.clone();

            // self.rename_entry(&old_path, &new_path).await?;
            // *entry.path = new_path.into();

            let old_path = entry.path_rx.borrow().clone();
            let new_path = update_path_parent(&old_path, &new_parent)?;

            entry
                .edit
                .rename(&self.abs_path, &old_path, &new_path)
                .await?;
        }

        if let Some(name) = &params.name {
            // let old_path = entry.path.clone();
            // let new_path = rename_path(entry.path.as_ref(), &name);
            // path = new_path.clone();

            // self.rename_entry(&old_path, &new_path).await?;
            // *entry.path = new_path.into();

            let old_path = entry.path_rx.borrow().clone();
            let new_path = rename_path(&old_path, name);

            entry
                .edit
                .rename(&self.abs_path, &old_path, &new_path)
                .await?;
        }

        self.patch_item_entry(entry, &params).await?;

        let path = entry.path_rx.borrow().clone();
        drop(state_lock);

        // if let Some(protocol) = params.protocol {
        //     match entry.configuration {
        //         RawItemConfiguration::Request(request) => {
        //             request.change_protocol(protocol);

        //             Ok(())
        //         }
        //         RawItemConfiguration::Endpoint(endpoint) => {
        //             endpoint.change_protocol(protocol);

        //             Ok(())
        //         }
        //         RawItemConfiguration::Component(_) => {
        //             Err(joinerror::Error::new::<ErrorInvalidInput>(
        //                 "cannot set protocol for component item",
        //             ))
        //         }
        //         RawItemConfiguration::Schema(_) => Err(joinerror::Error::new::<ErrorInvalidInput>(
        //             "cannot set protocol for schema item",
        //         )),
        //     }?;
        // }

        // let configuration = entry.configuration.clone();
        let is_db_update_needed = params.order.is_some() || params.expanded.is_some();
        if !is_db_update_needed {
            return Ok(path);
        }

        let mut txn = self.storage.begin_write(ctx).await?;

        if let Some(order) = params.order {
            self.storage
                .put_entry_order_txn(ctx, &mut txn, id, order)
                .await?;
        }

        let mut state_lock = self.state.write().await;
        if let Some(expanded) = params.expanded {
            if expanded {
                state_lock.expanded_entries.insert(id.to_owned());
            } else {
                state_lock.expanded_entries.remove(id);
            }

            self.storage
                .put_expanded_entries_txn(
                    ctx,
                    &mut txn,
                    state_lock
                        .expanded_entries
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                )
                .await?;
        }

        txn.commit()?;

        Ok(path)
    }

    async fn patch_item_entry(
        &self,
        entry: &mut Entry,
        params: &ModifyParams,
    ) -> joinerror::Result<()> {
        let mut on_edit_success = Vec::new();
        let mut patches = Vec::new();

        if let Some(protocol) = &params.protocol {
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/url/protocol") },
                    value: JsonValue::String(protocol.to_string()),
                }),
                EditOptions {
                    create_missing_segments: false,
                    ignore_if_not_exists: false,
                },
            ));
            on_edit_success.push(|| {
                entry.protocol = Some(protocol.clone());
            });
        }

        // TODO: handle other stuff

        if patches.is_empty() {
            return Ok(());
        }

        entry.edit.edit(&self.abs_path, &patches).await?;

        for mut callback in on_edit_success {
            callback();
        }

        Ok(())
    }
}

impl<R: AppRuntime> Worktree<R> {
    pub fn new(
        abs_path: Arc<Path>,
        fs: Arc<dyn FileSystem>,
        storage: Arc<StorageService<R>>,
    ) -> Self {
        Self {
            abs_path,
            fs,
            storage,
            state: Default::default(),
        }
    }
}

impl<R: AppRuntime> Worktree<R> {
    async fn create_entry(
        &self,
        path: &SanitizedPath,
        is_dir: bool,
        content: &[u8],
    ) -> joinerror::Result<()> {
        let abs_path = self.absolutize(&path)?;
        if abs_path.exists() {
            return Err(joinerror::Error::new::<ErrorAlreadyExists>(format!(
                "entry already exists: {}",
                abs_path.display()
            )));
        }

        self.fs.create_dir(&abs_path).await?;

        let file_path = if is_dir {
            abs_path.join(constants::DIR_CONFIG_FILENAME)
        } else {
            abs_path.join(constants::ITEM_CONFIG_FILENAME)
        };

        self.fs
            .create_file_with(
                &file_path,
                content,
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        Ok(())
    }

    // async fn rename_entry(&self, from: &Path, to: &Path) -> joinerror::Result<()> {
    //     // On Windows and macOS, file/directory names are case-preserving but insensitive
    //     // If the from and to path differs only with different casing of the filename
    //     // The rename should still succeed

    //     let abs_from = self.absolutize(&from)?;
    //     let abs_to = self.absolutize(&to)?;

    //     if !abs_from.exists() {
    //         return Err(joinerror::Error::new::<ErrorNotFound>(format!(
    //             "entry not found: {}",
    //             abs_from.display()
    //         )));
    //     }

    //     let old_name_lower = from
    //         .file_name()
    //         .map(|name| name.to_string_lossy().to_lowercase())
    //         .ok_or_join_err::<ErrorIo>("invalid file name")?;
    //     let new_name_lower = to
    //         .file_name()
    //         .map(|name| name.to_string_lossy().to_lowercase())
    //         .ok_or_join_err::<ErrorIo>("invalid file name")?;
    //     let recasing_only =
    //         old_name_lower == new_name_lower && abs_from.parent() == abs_to.parent();

    //     if abs_to.exists() && !recasing_only {
    //         return Err(joinerror::Error::new::<ErrorAlreadyExists>(format!(
    //             "entry already exists: {}",
    //             to.display()
    //         )));
    //     }

    //     self.fs
    //         .rename(
    //             &abs_from,
    //             &abs_to,
    //             moss_fs::RenameOptions {
    //                 overwrite: true,
    //                 ignore_if_exists: false,
    //             },
    //         )
    //         .await?;

    //     Ok(())
    // }
}

/// Update the filename of a path to the encoded input name
fn rename_path(path: &Path, name: &str) -> PathBuf {
    let mut buf = PathBuf::from(path);
    buf.pop();
    buf.push(sanitize(name));
    buf
}

/// Update the parent of a path, preserving the filename
fn update_path_parent(path: &Path, new_parent: &Path) -> anyhow::Result<PathBuf> {
    let name = path
        .file_name()
        .ok_or(anyhow!(format!(
            "Invalid entry path: {}",
            path.to_string_lossy().to_string()
        )))?
        .to_string_lossy()
        .to_string();

    Ok(new_parent.join(name))
}

// We don't allow creating subentries inside an item
fn is_parent_dir_entry(abs_path: &Path, parent_path: &Path) -> bool {
    if parent_path == Path::new(COLLECTION_ROOT_PATH) {
        // Ignore the root level since it's not an entry
        return true;
    }
    abs_path
        .join(parent_path)
        .join(constants::DIR_CONFIG_FILENAME)
        .exists()
}

async fn process_entry(
    path: Arc<Path>,
    all_entry_keys: &HashMap<SegKeyBuf, AnyValue>,
    expanded_entries: &HashSet<EntryId>,
    fs: &Arc<dyn FileSystem>,
    abs_path: &Path,
) -> joinerror::Result<Option<(Entry, EntryDescription)>> {
    let dir_config_path = abs_path.join(constants::DIR_CONFIG_FILENAME);
    let item_config_path = abs_path.join(constants::ITEM_CONFIG_FILENAME);

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());

    if dir_config_path.exists() {
        let mut rdr = fs.open_file(&dir_config_path).await?;
        let model: EntryModel =
            hcl::from_reader(&mut rdr).join_err::<()>("failed to parse dir configuration")?;

        let id = model.id().clone();
        let desc = EntryDescription {
            id: id.clone(),
            name: desanitize(&name),
            path: path.clone(),
            class: model.class(),
            kind: EntryKind::Dir,
            protocol: None,
            order: all_entry_keys
                .get(&segments::segkey_entry_order(&id))
                .and_then(|o| o.deserialize().ok()),
            expanded: expanded_entries.contains(&id),
        };
        let (path_tx, path_rx) = watch::channel(desanitize_path(&path, None)?.into());

        return Ok(Some((
            Entry {
                id,
                // path: desanitize_path(path, None)?.into(),
                path_rx,
                edit: EntryEditing::new(fs.clone(), path_tx),
                class: model.class(),
                protocol: None,
                // configuration: EntryConfiguration::Dir(config),
            },
            desc,
        )));
    }

    if item_config_path.exists() {
        let mut rdr = fs.open_file(&item_config_path).await?;
        let model: EntryModel =
            hcl::from_reader(&mut rdr).join_err::<()>("failed to parse item configuration")?;

        let id = model.id().clone();
        let desc = EntryDescription {
            id: id.clone(),
            name: desanitize(&name),
            path: path.clone(),
            class: model.class(),
            kind: EntryKind::Item,
            protocol: model.protocol(),
            order: all_entry_keys
                .get(&segments::segkey_entry_order(&id))
                .and_then(|o| o.deserialize().ok()),
            expanded: expanded_entries.contains(&id),
        };

        let (path_tx, path_rx) = watch::channel(desanitize_path(&path, None)?.into());

        return Ok(Some((
            Entry {
                id,
                // path: desanitize_path(path, None)?.into(),
                // configuration: EntryConfiguration::Item(config),
                path_rx,
                edit: EntryEditing::new(fs.clone(), path_tx),
                class: model.class(),
                protocol: model.protocol(),
            },
            desc,
        )));
    }

    Ok(None)
}

async fn process_file(
    _name: &str,
    _path: &Arc<Path>,
    _fs: &Arc<dyn FileSystem>,
    _abs_path: &Path,
) -> joinerror::Result<Option<(Entry, EntryDescription)>> {
    // TODO: implement
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_patch::{
        MoveOperation, PatchOperation, RemoveOperation, ReplaceOperation, jsonptr::PointerBuf,
    };
    use moss_edit::json::{EditOptions, JsonEdit};
    use serde_json::{Value as JsonValue, json};

    #[test]
    fn patch_test() {
        let content = r#"
            request {
                metadata {
                    id = "en81rfzU5M"
                }

                get {
                    raw = "test"
                }
            }
        "#;

        let mut edits = JsonEdit::new();
        let mut value: JsonValue = hcl::from_str(content).unwrap();

        // let p = unsafe { PointerBuf::new_unchecked("/request/get") };
        // let v = value.pointer(&p).unwrap();

        edits
            .apply(
                &mut value,
                &[(
                    PatchOperation::Move(MoveOperation {
                        from: unsafe { PointerBuf::new_unchecked("/request/get") },
                        path: unsafe { PointerBuf::new_unchecked("/request/post") },
                    }),
                    EditOptions {
                        ignore_if_not_exists: false,
                        create_missing_segments: false,
                    },
                )],
            )
            .unwrap();

        dbg!(&value);
    }
}
