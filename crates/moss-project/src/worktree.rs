pub mod entry;

use anyhow::anyhow;
use joinerror::OptionExt;
use json_patch::{PatchOperation, ReplaceOperation, jsonptr::PointerBuf};
use moss_app_delegate::{AppDelegate, broadcast::ToLocation};
use moss_applib::AppRuntime;
use moss_common::{continue_if_err, continue_if_none};
use moss_db::primitives::AnyValue;
use moss_edit::json::EditOptions;
use moss_fs::{CreateOptions, FileSystem, RemoveOptions, desanitize_path, utils::SanitizedPath};
use moss_hcl::HclResultExt;
use moss_storage::primitives::segkey::SegKeyBuf;
use moss_text::sanitized::{desanitize, sanitize};
use serde_json::Value as JsonValue;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    fs,
    sync::{RwLock, mpsc, watch},
};

use crate::{
    constants::{self, DIR_CONFIG_FILENAME},
    dirs,
    errors::{ErrorAlreadyExists, ErrorInvalidInput, ErrorNotFound},
    models::primitives::{EntryId, EntryKind, EntryProtocol},
    services::storage_service::StorageService,
    storage::segments,
    worktree::entry::{Entry, EntryDescription, edit::EntryEditing, model::EntryModel},
};

trait IsRoot {
    fn is_root(&self) -> bool;
}

impl IsRoot for Path {
    fn is_root(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

impl IsRoot for PathBuf {
    fn is_root(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

#[derive(Debug)]
struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    scan_queue: mpsc::UnboundedSender<ScanJob>,
}

pub(crate) struct ModifyParams {
    pub name: Option<String>,
    pub protocol: Option<EntryProtocol>,
    pub expanded: Option<bool>,
    pub order: Option<isize>,
    pub path: Option<PathBuf>,
    //
    //TODO: Add
    //
    // pub query_params_to_add: Vec<AddQueryParamParams>,
    // pub query_params_to_update: Vec<UpdateQueryParamParams>,
    // pub query_params_to_remove: Vec<QueryParamId>,

    // pub path_params_to_add: Vec<AddPathParamParams>,
    // pub path_params_to_update: Vec<UpdatePathParamParams>,
    // pub path_params_to_remove: Vec<PathParamId>,

    // pub headers_to_add: Vec<AddHeaderParams>,
    // pub headers_to_update: Vec<UpdateHeaderParams>,
    // pub headers_to_remove: Vec<HeaderParamId>,
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

// Required for OnceCell::set
impl<R: AppRuntime> Debug for Worktree<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Worktree")
            .field("abs_path", &self.abs_path)
            .finish()
    }
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
            Ok(self.abs_path.join(dirs::RESOURCES_DIR).join(path))
        } else {
            Ok(self.abs_path.join(dirs::RESOURCES_DIR).to_path_buf())
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
        app_delegate: AppDelegate<R>,
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

        let activity_handle = app_delegate.emit_continual(ToLocation::Window {
            activity_id: "scan_worktree",
            title: "Scanning".to_string(),
            detail: None,
        })?;

        let mut handles = Vec::new();
        while let Some(job) = job_rx.recv().await {
            let sender = sender.clone();
            let fs = self.fs.clone();
            let state = self.state.clone();
            let expanded_entries = expanded_entries.clone();
            let all_entry_keys = all_entry_keys.clone();

            activity_handle.emit_progress(Some(job.path.display().to_string()))?;

            let handle = tokio::spawn(async move {
                let mut new_jobs = Vec::new();

                if !job.path.as_os_str().is_empty() {
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

        activity_handle.emit_finish()?;

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

        let sanitized_path: SanitizedPath = moss_fs::utils::sanitize_path(path, None)?
            .join(sanitize(name))
            .into();

        let content = hcl::to_string(&model)
            .join_err::<()>("failed to serialize configuration into hcl string")?;
        self.create_entry_internal(&sanitized_path, false, &content.as_bytes())
            .await?;

        let mut state_lock = self.state.write().await;

        let id = model.id().clone();
        let (path_tx, path_rx) = watch::channel(sanitized_path.to_path_buf().into());

        state_lock.entries.insert(
            id.clone(),
            Entry {
                id: id.clone(),
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

        let sanitized_path: SanitizedPath = moss_fs::utils::sanitize_path(path, None)?
            .join(sanitize(name))
            .into();

        let content = hcl::to_string(&model)
            .join_err::<()>("failed to serialize configuration into hcl string")?;
        self.create_entry_internal(&sanitized_path, true, &content.as_bytes())
            .await?;

        let mut state_lock = self.state.write().await;

        let id = model.id().clone();
        let (path_tx, path_rx) = watch::channel(sanitized_path.to_path_buf().into());
        state_lock.entries.insert(
            id.clone(),
            Entry {
                id: id.clone(),
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

        if let Some(new_parent) = params.path {
            if !new_parent.is_root() {
                // For now, we can only move entries into a directory entry
                // Check if the destination path has dir config file
                let dest_abs_path = self.absolutize(&new_parent.join(DIR_CONFIG_FILENAME))?;
                if !dest_abs_path.exists() {
                    return Err(joinerror::Error::new::<ErrorInvalidInput>(
                        "cannot move entries into a non-directory entry",
                    ));
                }
            }

            let old_path = entry.path_rx.borrow().clone();
            let new_path = update_path_parent(&old_path, &new_parent)?;

            entry
                .edit
                .rename(
                    &self.abs_path.join(dirs::RESOURCES_DIR),
                    &old_path,
                    &new_path,
                )
                .await?;
        }

        if let Some(name) = params.name {
            let old_path = entry.path_rx.borrow().clone();
            let new_path = rename_path(&old_path, &name);

            entry
                .edit
                .rename(
                    &self.abs_path.join(dirs::RESOURCES_DIR),
                    &old_path,
                    &new_path,
                )
                .await?;
        }

        // TODO: patch the dir entry

        let path = entry.path_rx.borrow().clone();
        drop(state_lock);

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

        if let Some(new_parent) = &params.path {
            if !new_parent.is_root() {
                // For now, we can only move entries into a directory entry
                // Check if the destination path has dir config file
                let dest_abs_path = self.absolutize(&new_parent.join(DIR_CONFIG_FILENAME))?;
                if !dest_abs_path.exists() {
                    return Err(joinerror::Error::new::<ErrorInvalidInput>(
                        "cannot move entries into a non-directory entry",
                    ));
                }
            }

            let old_path = entry.path_rx.borrow().clone();
            let new_path = update_path_parent(&old_path, &new_parent)?;

            entry
                .edit
                .rename(
                    &self.abs_path.join(dirs::RESOURCES_DIR),
                    &old_path,
                    &new_path,
                )
                .await?;
        }

        if let Some(name) = &params.name {
            let old_path = entry.path_rx.borrow().clone();
            let new_path = rename_path(&old_path, name);

            entry
                .edit
                .rename(
                    &self.abs_path.join(dirs::RESOURCES_DIR),
                    &old_path,
                    &new_path,
                )
                .await?;
        }

        self.patch_item_entry(entry, &params).await?;

        let path = entry.path_rx.borrow().clone();
        drop(state_lock);

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
    async fn create_entry_internal(
        &self,
        path: &SanitizedPath,
        is_dir: bool,
        content: &[u8],
    ) -> joinerror::Result<()> {
        let abs_path = self.absolutize(&path.to_path_buf())?;
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
                path_rx,
                edit: EntryEditing::new(fs.clone(), path_tx),
                class: model.class(),
                protocol: None,
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
