pub mod entry;

use anyhow::anyhow;
use hcl::ser::LabeledBlock;
use indexmap::IndexMap;
use joinerror::{Error, OptionExt};
use json_patch::{
    AddOperation, PatchOperation, RemoveOperation, ReplaceOperation, jsonptr::PointerBuf,
};
use moss_app_delegate::{AppDelegate, broadcast::ToLocation};
use moss_applib::AppRuntime;
use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use moss_common::{continue_if_err, continue_if_none};
use moss_db::primitives::AnyValue;
use moss_edit::json::EditOptions;
use moss_fs::{CreateOptions, FileSystem, RemoveOptions, desanitize_path, utils::SanitizedPath};
use moss_hcl::{HclResultExt, hcl_to_json, json_to_hcl};
use moss_logging::session;
use moss_storage::primitives::segkey::SegKeyBuf;
use moss_text::sanitized::{desanitize, sanitize};
use serde_json::{Value as JsonValue, json};
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
    constants::{self, DIR_CONFIG_FILENAME, ITEM_CONFIG_FILENAME},
    dirs,
    dirs::RESOURCES_DIR,
    errors::{ErrorAlreadyExists, ErrorInvalidInput, ErrorNotFound},
    models::{
        operations::DescribeResourceOutput,
        primitives::{
            FormDataParamId, HeaderId, PathParamId, QueryParamId, ResourceId, ResourceKind,
            ResourceProtocol, UrlencodedParamId,
        },
        types::{
            BodyInfo, FormDataParamInfo, HeaderInfo, PathParamInfo, QueryParamInfo,
            UpdateBodyParams, UrlencodedParamInfo,
            http::{
                AddHeaderParams, AddPathParamParams, AddQueryParamParams, UpdateHeaderParams,
                UpdatePathParamParams, UpdateQueryParamParams,
            },
        },
    },
    storage::{
        StorageService, segments,
        segments::{
            segkey_entry_body_formdata_param_order, segkey_entry_body_urlencoded_param_order,
            segkey_entry_header_order, segkey_entry_path_param_order,
            segkey_entry_query_param_order,
        },
    },
    worktree::entry::{
        Entry, EntryDescription, EntryMetadata,
        edit::EntryEditing,
        model::{
            BodyKind, BodySpec, EntryModel, FormDataParamSpec, FormDataParamSpecOptions,
            HeaderParamSpec, HeaderParamSpecOptions, PathParamSpec, PathParamSpecOptions,
            QueryParamSpec, QueryParamSpecOptions, UrlencodedParamSpec, UrlencodedParamSpecOptions,
        },
    },
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
    pub protocol: Option<ResourceProtocol>,
    pub expanded: Option<bool>,
    pub order: Option<isize>,
    pub path: Option<PathBuf>,

    pub headers_to_add: Vec<AddHeaderParams>,
    pub headers_to_update: Vec<UpdateHeaderParams>,
    pub headers_to_remove: Vec<HeaderId>,

    pub path_params_to_add: Vec<AddPathParamParams>,
    pub path_params_to_update: Vec<UpdatePathParamParams>,
    pub path_params_to_remove: Vec<PathParamId>,

    pub query_params_to_add: Vec<AddQueryParamParams>,
    pub query_params_to_update: Vec<UpdateQueryParamParams>,
    pub query_params_to_remove: Vec<QueryParamId>,

    pub body: Option<UpdateBodyParams>,
}

#[derive(Default)]
struct WorktreeState {
    entries: HashMap<ResourceId, Entry>,
    expanded_entries: HashSet<ResourceId>,
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

    pub async fn remove_entry(
        &self,
        ctx: &R::AsyncContext,
        id: &ResourceId,
    ) -> joinerror::Result<()> {
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
        expanded_entries: Arc<HashSet<ResourceId>>,
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
                edit: EntryEditing::new(self.fs.clone(), path_tx, ITEM_CONFIG_FILENAME),
                class: model.metadata.class.clone(),
                protocol: model.protocol(),
                metadata: EntryMetadata {
                    body_kind: model.body_kind(),
                },
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
                edit: EntryEditing::new(self.fs.clone(), path_tx, DIR_CONFIG_FILENAME),
                class: model.metadata.class.clone(),
                protocol: None,
                metadata: EntryMetadata { body_kind: None },
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
        id: &ResourceId,
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
        app_delegate: &AppDelegate<R>,
        id: &ResourceId,
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

        self.patch_item_entry(ctx, app_delegate, entry, &params)
            .await?;

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

    pub async fn describe_entry(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        id: &ResourceId,
    ) -> joinerror::Result<DescribeResourceOutput> {
        let state_lock = self.state.read().await;
        let entry = state_lock
            .entries
            .get(&id)
            .ok_or_join_err_with::<ErrorNotFound>(|| format!("entry {} not found", id))?;
        let entry_path = self
            .abs_path
            .join(RESOURCES_DIR)
            .join(entry.path_rx.borrow().as_ref());
        let dir_config_path = entry_path.join(constants::DIR_CONFIG_FILENAME);
        let item_config_path = entry_path.join(constants::ITEM_CONFIG_FILENAME);

        let name = entry_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| entry_path.to_string_lossy().to_string());

        if dir_config_path.exists() {
            let mut rdr = self.fs.open_file(&dir_config_path).await?;
            let model: EntryModel =
                hcl::from_reader(&mut rdr).join_err::<()>("failed to parse dir configuration")?;

            return Ok(DescribeResourceOutput {
                name: desanitize(&name),
                class: model.class(),
                kind: ResourceKind::Dir,
                protocol: None,
                url: None,
                headers: vec![],
                path_params: vec![],
                query_params: vec![],
                body: None,
            });
        } else if item_config_path.exists() {
            let entry_keys = self
                .storage
                .get_entry_keys(ctx, id)
                .await
                .unwrap_or_else(|err| {
                    session::error!(format!("failed to get entry cache: {}", err.to_string()));
                    HashMap::new()
                });

            let mut rdr = self.fs.open_file(&item_config_path).await?;
            let model: EntryModel =
                hcl::from_reader(&mut rdr).join_err::<()>("failed to parse item configuration")?;
            let class = model.class();
            let protocol = model.protocol();
            let url = model.url.map(|url| url.raw.clone());

            let mut header_infos = Vec::new();
            let mut path_param_infos = Vec::new();
            let mut query_param_infos = Vec::new();

            if let Some(header_block) = model.headers {
                for (header_id, header_spec) in header_block.into_inner() {
                    let value = match hcl_to_json(&header_spec.value) {
                        Ok(value) => value,
                        Err(err) => {
                            session::error!(format!(
                                "failed to convert value expression `{}`, {}",
                                &header_spec.value,
                                err.to_string()
                            ));
                            let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                                activity_id: "expression_conversion_error",
                                title: "Failed to convert value expression".to_string(),
                                detail: Some(err.to_string()),
                            });
                            JsonValue::Null
                        }
                    };
                    header_infos.push(HeaderInfo {
                        id: header_id.clone(),
                        name: header_spec.name,
                        value,
                        description: header_spec.description,
                        disabled: header_spec.options.disabled,
                        propagate: header_spec.options.propagate,
                        order: entry_keys
                            .get(&segkey_entry_header_order(id, &header_id))
                            .and_then(|value| value.deserialize().ok()),
                    })
                }
            }

            if let Some(path_param_block) = model.path_params {
                for (path_param_id, path_param_spec) in path_param_block.into_inner() {
                    let value = match hcl_to_json(&path_param_spec.value) {
                        Ok(value) => value,
                        Err(err) => {
                            session::error!(format!(
                                "failed to convert value expression `{}`, {}",
                                &path_param_spec.value,
                                err.to_string()
                            ));
                            let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                                activity_id: "expression_conversion_error",
                                title: "Failed to convert value expression".to_string(),
                                detail: Some(err.to_string()),
                            });
                            JsonValue::Null
                        }
                    };

                    path_param_infos.push(PathParamInfo {
                        id: path_param_id.clone(),
                        name: path_param_spec.name,
                        value,
                        description: path_param_spec.description,
                        disabled: path_param_spec.options.disabled,
                        propagate: path_param_spec.options.propagate,
                        order: entry_keys
                            .get(&segkey_entry_path_param_order(id, &path_param_id))
                            .and_then(|value| value.deserialize().ok()),
                    })
                }
            }

            if let Some(query_param_block) = model.query_params {
                for (query_param_id, query_param_spec) in query_param_block.into_inner() {
                    let value = match hcl_to_json(&query_param_spec.value) {
                        Ok(value) => value,
                        Err(err) => {
                            session::error!(format!(
                                "failed to convert value expression `{}`, {}",
                                &query_param_spec.value,
                                err.to_string()
                            ));
                            let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                                activity_id: "expression_conversion_error",
                                title: "Failed to convert value expression".to_string(),
                                detail: Some(err.to_string()),
                            });
                            JsonValue::Null
                        }
                    };

                    query_param_infos.push(QueryParamInfo {
                        id: query_param_id.clone(),
                        name: query_param_spec.name,
                        value,
                        description: query_param_spec.description,
                        disabled: query_param_spec.options.disabled,
                        propagate: query_param_spec.options.propagate,
                        order: entry_keys
                            .get(&segkey_entry_query_param_order(id, &query_param_id))
                            .and_then(|value| value.deserialize().ok()),
                    })
                }
            }

            let body_info = if let Some(body) = model.body {
                describe_body(app_delegate, id, body, &entry_keys).await
            } else {
                None
            };

            return Ok(DescribeResourceOutput {
                name: desanitize(&name),
                class,
                kind: ResourceKind::Item,
                protocol,
                url,
                headers: header_infos,
                path_params: path_param_infos,
                query_params: query_param_infos,
                body: body_info,
            });
        } else {
            return Err(Error::new::<()>("cannot find entry config"));
        }
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

    async fn patch_item_entry(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        entry: &mut Entry,
        params: &ModifyParams,
    ) -> joinerror::Result<()> {
        let mut on_edit_success: Vec<Box<dyn FnOnce(&mut Entry) + Send>> = Vec::new();
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

            let protocol_clone = protocol.clone();
            on_edit_success.push(Box::new(move |entry: &mut Entry| {
                entry.protocol = Some(protocol_clone)
            }));
        }

        // TODO: Extract order storage into one database transaction?

        for header_to_add in &params.headers_to_add {
            let id = HeaderId::new();
            let id_str = id.to_string();

            let value = continue_if_err!(json_to_hcl(&header_to_add.value), |err| {
                session::error!("failed to convert value expression: {}", err);
                let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                    activity_id: "expression_conversion_error",
                    title: "Failed to convert value expression".to_string(),
                    detail: Some(err),
                });
            });

            let spec = HeaderParamSpec {
                name: header_to_add.name.clone(),
                value,
                description: header_to_add.description.clone(),
                options: HeaderParamSpecOptions {
                    disabled: header_to_add.options.disabled,
                    propagate: header_to_add.options.propagate,
                },
            };

            let spec_value = continue_if_err!(
                serde_json::to_value(&spec).map_err(|e| e.to_string()),
                |err| {
                    session::error!(format!("failed to convert header spec to json: {}", err));
                    let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                        activity_id: "header_spec_conversion_error",
                        title: "Failed to convert header spec to json".to_string(),
                        detail: Some(err),
                    });
                }
            );

            patches.push((
                PatchOperation::Add(AddOperation {
                    path: unsafe { PointerBuf::new_unchecked(format!("/header/{}", id_str)) },
                    value: spec_value,
                }),
                EditOptions {
                    create_missing_segments: true,
                    ignore_if_not_exists: false,
                },
            ));

            // We don't want database failure to stop the function
            let mut txn = continue_if_err!(self.storage.begin_write(ctx).await, |err| {
                session::error!(format!("failed to start a write transaction: {}", err))
            });

            continue_if_err!(
                self.storage
                    .put_entry_header_order_txn(ctx, &mut txn, &entry.id, &id, header_to_add.order)
                    .await,
                |err| { session::error!(format!("failed to put header order: {}", err)) }
            );
            continue_if_err!(txn.commit(), |err| {
                session::error!(format!("failed to commit transaction: {}", err))
            });
        }

        for header_to_update in &params.headers_to_update {
            if let Some(new_name) = &header_to_update.name {
                patches.push((
                    PatchOperation::Replace(ReplaceOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/header/{}/name",
                                header_to_update.id
                            ))
                        },
                        value: JsonValue::String(new_name.clone()),
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
            }

            match &header_to_update.value {
                Some(ChangeJsonValue::Update(value)) => {
                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/header/{}/value",
                                    header_to_update.id
                                ))
                            },
                            value: value.clone(),
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                Some(ChangeJsonValue::Remove) => {
                    patches.push((
                        PatchOperation::Remove(RemoveOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/header/{}/value",
                                    header_to_update.id
                                ))
                            },
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                _ => {}
            }

            match &header_to_update.description {
                Some(ChangeString::Update(value)) => {
                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/header/{}/description",
                                    header_to_update.id
                                ))
                            },
                            value: JsonValue::String(value.clone()),
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                Some(ChangeString::Remove) => {
                    patches.push((
                        PatchOperation::Remove(RemoveOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/header/{}/description",
                                    header_to_update.id
                                ))
                            },
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                _ => {}
            }

            if let Some(options) = &header_to_update.options {
                let options = HeaderParamSpecOptions {
                    disabled: options.disabled,
                    propagate: options.propagate,
                };
                let options_value = continue_if_err!(serde_json::to_value(options), |err| {
                    session::error!(format!("failed to convert options value: {}", err))
                });

                patches.push((
                    PatchOperation::Replace(ReplaceOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/header/{}/options",
                                header_to_update.id
                            ))
                        },
                        value: options_value,
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
            }

            if let Some(order) = header_to_update.order {
                // We don't want database failure to stop the function
                let mut txn = continue_if_err!(self.storage.begin_write(ctx).await, |err| {
                    session::error!(format!("failed to start a write transaction: {}", err))
                });

                continue_if_err!(
                    self.storage
                        .put_entry_header_order_txn(
                            ctx,
                            &mut txn,
                            &entry.id,
                            &header_to_update.id,
                            order
                        )
                        .await,
                    |err| { session::error!(format!("failed to put header order: {}", err)) }
                );
                continue_if_err!(txn.commit(), |err| {
                    session::error!(format!("failed to commit transaction: {}", err))
                });
            }
        }

        for id in &params.headers_to_remove {
            patches.push((
                PatchOperation::Remove(RemoveOperation {
                    path: unsafe { PointerBuf::new_unchecked(format!("/header/{}", id)) },
                }),
                EditOptions {
                    create_missing_segments: false,
                    ignore_if_not_exists: false,
                },
            ));

            // We don't want database failure to stop the function
            let mut txn = continue_if_err!(self.storage.begin_write(ctx).await, |err| {
                session::error!(format!("failed to start a write transaction: {}", err))
            });

            continue_if_err!(
                self.storage
                    .remove_entry_header_order_txn(ctx, &mut txn, &entry.id, &id,)
                    .await,
                |err| { session::error!(format!("failed to remove header order: {}", err)) }
            );
            continue_if_err!(txn.commit(), |err| {
                session::error!(format!("failed to commit transaction: {}", err))
            });
        }

        for path_param_to_add in &params.path_params_to_add {
            let id = PathParamId::new();
            let id_str = id.to_string();

            let value = continue_if_err!(json_to_hcl(&path_param_to_add.value), |err| {
                session::error!("failed to convert value expression: {}", err);
                let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                    activity_id: "expression_conversion_error",
                    title: "Failed to convert value expression".to_string(),
                    detail: Some(err),
                });
            });

            let spec = PathParamSpec {
                name: path_param_to_add.name.clone(),
                value,
                description: path_param_to_add.description.clone(),
                options: PathParamSpecOptions {
                    disabled: path_param_to_add.options.disabled,
                    propagate: path_param_to_add.options.propagate,
                },
            };

            let spec_value = continue_if_err!(
                serde_json::to_value(&spec).map_err(|err| err.to_string()),
                |err| {
                    session::error!(format!(
                        "failed to convert path param spec to json: {}",
                        err
                    ));
                    let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                        activity_id: "expression_conversion_error",
                        title: "Failed to convert value expression".to_string(),
                        detail: Some(err),
                    });
                }
            );

            patches.push((
                PatchOperation::Add(AddOperation {
                    path: unsafe { PointerBuf::new_unchecked(format!("/path_param/{}", id_str)) },
                    value: spec_value,
                }),
                EditOptions {
                    create_missing_segments: true,
                    ignore_if_not_exists: false,
                },
            ));

            // We don't want database failure to stop the function
            let mut txn = continue_if_err!(self.storage.begin_write(ctx).await, |err| {
                session::error!(format!("failed to start a write transaction: {}", err))
            });

            continue_if_err!(
                self.storage
                    .put_entry_path_param_order_txn(
                        ctx,
                        &mut txn,
                        &entry.id,
                        &id,
                        path_param_to_add.order
                    )
                    .await,
                |err| { session::error!(format!("failed to put path param order: {}", err)) }
            );
            continue_if_err!(txn.commit(), |err| {
                session::error!(format!("failed to commit transaction: {}", err))
            });
        }

        for path_param_to_update in &params.path_params_to_update {
            if let Some(new_name) = &path_param_to_update.name {
                patches.push((
                    PatchOperation::Replace(ReplaceOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/path_param/{}/name",
                                path_param_to_update.id
                            ))
                        },
                        value: JsonValue::String(new_name.clone()),
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
            }

            match &path_param_to_update.value {
                Some(ChangeJsonValue::Update(value)) => {
                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/path_param/{}/value",
                                    path_param_to_update.id
                                ))
                            },
                            value: value.clone(),
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                Some(ChangeJsonValue::Remove) => {
                    patches.push((
                        PatchOperation::Remove(RemoveOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/path_param/{}/value",
                                    path_param_to_update.id
                                ))
                            },
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                _ => {}
            }

            match &path_param_to_update.description {
                Some(ChangeString::Update(value)) => {
                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/path_param/{}/description",
                                    path_param_to_update.id
                                ))
                            },
                            value: JsonValue::String(value.clone()),
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                Some(ChangeString::Remove) => {
                    patches.push((
                        PatchOperation::Remove(RemoveOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/path_param/{}/description",
                                    path_param_to_update.id
                                ))
                            },
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                _ => {}
            }

            if let Some(options) = &path_param_to_update.options {
                let options = PathParamSpecOptions {
                    disabled: options.disabled,
                    propagate: options.propagate,
                };
                let options_value = continue_if_err!(serde_json::to_value(options), |err| {
                    session::error!(format!("failed to convert options value: {}", err))
                });

                patches.push((
                    PatchOperation::Replace(ReplaceOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/path_param/{}/options",
                                path_param_to_update.id
                            ))
                        },
                        value: options_value,
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
            }

            if let Some(order) = path_param_to_update.order {
                // We don't want database failure to stop the function
                let mut txn = continue_if_err!(self.storage.begin_write(ctx).await, |err| {
                    session::error!(format!("failed to start a write transaction: {}", err))
                });

                continue_if_err!(
                    self.storage
                        .put_entry_path_param_order_txn(
                            ctx,
                            &mut txn,
                            &entry.id,
                            &path_param_to_update.id,
                            order
                        )
                        .await,
                    |err| { session::error!(format!("failed to put path param order: {}", err)) }
                );
                continue_if_err!(txn.commit(), |err| {
                    session::error!(format!("failed to commit transaction: {}", err))
                });
            }
        }

        for id in &params.path_params_to_remove {
            patches.push((
                PatchOperation::Remove(RemoveOperation {
                    path: unsafe { PointerBuf::new_unchecked(format!("/path_param/{}", id)) },
                }),
                EditOptions {
                    create_missing_segments: false,
                    ignore_if_not_exists: false,
                },
            ));

            // We don't want database failure to stop the function
            let mut txn = continue_if_err!(self.storage.begin_write(ctx).await, |err| {
                session::error!(format!("failed to start a write transaction: {}", err))
            });

            continue_if_err!(
                self.storage
                    .remove_entry_path_param_order_txn(ctx, &mut txn, &entry.id, &id,)
                    .await,
                |err| { session::error!(format!("failed to remove path param order: {}", err)) }
            );
            continue_if_err!(txn.commit(), |err| {
                session::error!(format!("failed to commit transaction: {}", err))
            });
        }

        for query_param_to_add in &params.query_params_to_add {
            let id = QueryParamId::new();
            let id_str = id.to_string();

            let value = continue_if_err!(json_to_hcl(&query_param_to_add.value), |err| {
                session::error!("failed to convert value expression: {}", err);
                let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                    activity_id: "expression_conversion_error",
                    title: "Failed to convert value expression".to_string(),
                    detail: Some(err),
                });
            });

            let spec = QueryParamSpec {
                name: query_param_to_add.name.clone(),
                value,
                description: query_param_to_add.description.clone(),
                options: QueryParamSpecOptions {
                    disabled: query_param_to_add.options.disabled,
                    propagate: query_param_to_add.options.propagate,
                },
            };

            let spec_value = continue_if_err!(
                serde_json::to_value(&spec).map_err(|err| err.to_string()),
                |err| {
                    session::error!(format!(
                        "failed to convert query param spec to json: {}",
                        err
                    ));
                    let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                        activity_id: "query_param_spec_conversion_error",
                        title: "Failed to convert query param spec to json".to_string(),
                        detail: Some(err),
                    });
                }
            );

            patches.push((
                PatchOperation::Add(AddOperation {
                    path: unsafe { PointerBuf::new_unchecked(format!("/query_param/{}", id_str)) },
                    value: spec_value,
                }),
                EditOptions {
                    create_missing_segments: true,
                    ignore_if_not_exists: false,
                },
            ));

            // We don't want database failure to stop the function
            let mut txn = continue_if_err!(self.storage.begin_write(ctx).await, |err| {
                session::error!(format!("failed to start a write transaction: {}", err))
            });

            continue_if_err!(
                self.storage
                    .put_entry_query_param_order_txn(
                        ctx,
                        &mut txn,
                        &entry.id,
                        &id,
                        query_param_to_add.order
                    )
                    .await,
                |err| { session::error!(format!("failed to put query param order: {}", err)) }
            );
            continue_if_err!(txn.commit(), |err| {
                session::error!(format!("failed to commit transaction: {}", err))
            });
        }

        for query_param_to_update in &params.query_params_to_update {
            if let Some(new_name) = &query_param_to_update.name {
                patches.push((
                    PatchOperation::Replace(ReplaceOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/query_param/{}/name",
                                query_param_to_update.id
                            ))
                        },
                        value: JsonValue::String(new_name.clone()),
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
            }

            match &query_param_to_update.value {
                Some(ChangeJsonValue::Update(value)) => {
                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/query_param/{}/value",
                                    query_param_to_update.id
                                ))
                            },
                            value: value.clone(),
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                Some(ChangeJsonValue::Remove) => {
                    patches.push((
                        PatchOperation::Remove(RemoveOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/query_param/{}/value",
                                    query_param_to_update.id
                                ))
                            },
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                _ => {}
            }

            match &query_param_to_update.description {
                Some(ChangeString::Update(value)) => {
                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/query_param/{}/description",
                                    query_param_to_update.id
                                ))
                            },
                            value: JsonValue::String(value.clone()),
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                Some(ChangeString::Remove) => {
                    patches.push((
                        PatchOperation::Remove(RemoveOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "/query_param/{}/description",
                                    query_param_to_update.id
                                ))
                            },
                        }),
                        EditOptions {
                            // Raise an error if the variable does not exist
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }
                _ => {}
            }

            if let Some(options) = &query_param_to_update.options {
                let options = QueryParamSpecOptions {
                    disabled: options.disabled,
                    propagate: options.propagate,
                };
                let options_value = continue_if_err!(serde_json::to_value(options), |err| {
                    session::error!(format!("failed to convert options value: {}", err))
                });

                patches.push((
                    PatchOperation::Replace(ReplaceOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!(
                                "/query_param/{}/options",
                                query_param_to_update.id
                            ))
                        },
                        value: options_value,
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
            }

            if let Some(order) = query_param_to_update.order {
                // We don't want database failure to stop the function
                let mut txn = continue_if_err!(self.storage.begin_write(ctx).await, |err| {
                    session::error!(format!("failed to start a write transaction: {}", err))
                });

                continue_if_err!(
                    self.storage
                        .put_entry_query_param_order_txn(
                            ctx,
                            &mut txn,
                            &entry.id,
                            &query_param_to_update.id,
                            order
                        )
                        .await,
                    |err| { session::error!(format!("failed to put query param order: {}", err)) }
                );
                continue_if_err!(txn.commit(), |err| {
                    session::error!(format!("failed to commit transaction: {}", err))
                });
            }
        }

        for id in &params.query_params_to_remove {
            patches.push((
                PatchOperation::Remove(RemoveOperation {
                    path: unsafe { PointerBuf::new_unchecked(format!("/query_param/{}", id)) },
                }),
                EditOptions {
                    create_missing_segments: false,
                    ignore_if_not_exists: false,
                },
            ));

            // We don't want database failure to stop the function
            let mut txn = continue_if_err!(self.storage.begin_write(ctx).await, |err| {
                session::error!(format!("failed to start a write transaction: {}", err))
            });

            continue_if_err!(
                self.storage
                    .remove_entry_query_param_order_txn(ctx, &mut txn, &entry.id, id,)
                    .await,
                |err| { session::error!(format!("failed to remove query param order: {}", err)) }
            );
            continue_if_err!(txn.commit(), |err| {
                session::error!(format!("failed to commit transaction: {}", err))
            });
        }

        if let Some(body) = &params.body {
            let current_body_kind = entry.metadata.body_kind.clone();

            let new_body_kind = patch_item_body(
                self,
                ctx,
                app_delegate,
                current_body_kind.clone(),
                entry.id.clone(),
                &mut patches,
                body,
            )
            .await?;

            if new_body_kind != current_body_kind {
                let new_body_kind_clone = new_body_kind.clone();
                on_edit_success.push(Box::new(move |entry: &mut Entry| {
                    entry.metadata.body_kind = new_body_kind_clone;
                }));
            }
        }

        if patches.is_empty() {
            return Ok(());
        }

        entry
            .edit
            .edit(&self.abs_path.join(dirs::RESOURCES_DIR), &patches)
            .await?;

        for callback in on_edit_success {
            callback(entry);
        }

        Ok(())
    }
}

async fn patch_item_body<R: AppRuntime>(
    worktree: &Worktree<R>,
    ctx: &R::AsyncContext,
    app_delegate: &AppDelegate<R>,
    current_body_kind: Option<BodyKind>,
    entry_id: ResourceId,
    patches: &mut Vec<(PatchOperation, EditOptions)>,
    params: &UpdateBodyParams,
) -> joinerror::Result<Option<BodyKind>> {
    let new_body_kind = match params {
        UpdateBodyParams::Remove => {
            clear_item_body(worktree, ctx, entry_id, patches).await?;
            None
        }
        UpdateBodyParams::Text(text) => {
            if current_body_kind.is_some() && current_body_kind != Some(BodyKind::Text) {
                clear_item_body(worktree, ctx, entry_id, patches).await?;
            }

            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/body/text/text") },
                    value: JsonValue::String(text.to_owned()),
                }),
                EditOptions {
                    create_missing_segments: true,
                    ignore_if_not_exists: false,
                },
            ));
            Some(BodyKind::Text)
        }
        UpdateBodyParams::Json(json) => {
            if current_body_kind.is_some() && current_body_kind != Some(BodyKind::Json) {
                clear_item_body(worktree, ctx, entry_id, patches).await?;
            }
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/body/json/json") },
                    value: json.clone(),
                }),
                EditOptions {
                    create_missing_segments: true,
                    ignore_if_not_exists: false,
                },
            ));
            Some(BodyKind::Json)
        }
        UpdateBodyParams::Xml(xml) => {
            if current_body_kind.is_some() && current_body_kind != Some(BodyKind::Xml) {
                clear_item_body(worktree, ctx, entry_id, patches).await?;
            }
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/body/xml/xml") },
                    value: JsonValue::String(xml.to_owned()),
                }),
                EditOptions {
                    create_missing_segments: true,
                    ignore_if_not_exists: false,
                },
            ));
            Some(BodyKind::Xml)
        }
        UpdateBodyParams::Binary(path) => {
            if current_body_kind.is_some() && current_body_kind != Some(BodyKind::Binary) {
                clear_item_body(worktree, ctx, entry_id, patches).await?;
            }
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/body/binary/binary") },
                    value: JsonValue::String(path.to_string_lossy().to_string()),
                }),
                EditOptions {
                    create_missing_segments: true,
                    ignore_if_not_exists: false,
                },
            ));
            Some(BodyKind::Binary)
        }
        UpdateBodyParams::Urlencoded {
            params_to_add,
            params_to_update,
            params_to_remove,
        } => {
            let body_block = "/body/x-www-form-urlencoded";
            if current_body_kind.is_some() && current_body_kind != Some(BodyKind::Urlencoded) {
                clear_item_body(worktree, ctx, entry_id.clone(), patches).await?;
                // Create the body block even if no parameter is given
                patches.push((
                    PatchOperation::Add(AddOperation {
                        path: unsafe { PointerBuf::new_unchecked("/body") },
                        value: json!({
                            "x-www-form-urlencoded": {}
                        }),
                    }),
                    EditOptions {
                        create_missing_segments: true,
                        ignore_if_not_exists: false,
                    },
                ));
            }

            let mut param_order_updates = HashMap::new();
            let mut param_order_removes = Vec::new();

            for urlencoded_param_to_add in params_to_add {
                let id = urlencoded_param_to_add
                    .id
                    .clone()
                    .unwrap_or(UrlencodedParamId::new());
                let id_str = id.to_string();

                let value = continue_if_err!(json_to_hcl(&urlencoded_param_to_add.value), |err| {
                    session::error!(format!("failed to convert value expression: {}", err));
                    let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                        activity_id: "expression_conversion_error",
                        title: "Failed to convert value expression".to_string(),
                        detail: Some(err),
                    });
                });

                let spec = UrlencodedParamSpec {
                    name: urlencoded_param_to_add.name.clone(),
                    value,
                    description: urlencoded_param_to_add.description.clone(),
                    options: UrlencodedParamSpecOptions {
                        disabled: urlencoded_param_to_add.options.disabled,
                        propagate: urlencoded_param_to_add.options.propagate,
                    },
                };

                let spec_value = continue_if_err!(
                    serde_json::to_value(&spec).map_err(|err| err.to_string()),
                    |err| {
                        session::error!(format!(
                            "failed to convert urlencoded param spec to json: {}",
                            err
                        ));
                        let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                            activity_id: "urlencoded_param_spec_conversion_error",
                            title: "Failed to convert urlencoded param spec to json".to_string(),
                            detail: Some(err),
                        });
                    }
                );

                patches.push((
                    PatchOperation::Add(AddOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!("{body_block}/urlencoded/{id_str}"))
                        },
                        value: spec_value,
                    }),
                    EditOptions {
                        create_missing_segments: true,
                        ignore_if_not_exists: false,
                    },
                ));
                param_order_updates.insert(id, urlencoded_param_to_add.order);
            }

            for urlencoded_param_to_update in params_to_update {
                let id = urlencoded_param_to_update.id.clone();
                if let Some(new_name) = &urlencoded_param_to_update.name {
                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "{body_block}/urlencoded/{id}/name"
                                ))
                            },
                            value: JsonValue::String(new_name.clone()),
                        }),
                        EditOptions {
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }

                match &urlencoded_param_to_update.value {
                    Some(ChangeJsonValue::Update(value)) => {
                        patches.push((
                            PatchOperation::Replace(ReplaceOperation {
                                path: unsafe {
                                    PointerBuf::new_unchecked(format!(
                                        "{body_block}/urlencoded/{id}/value"
                                    ))
                                },
                                value: value.clone(),
                            }),
                            EditOptions {
                                create_missing_segments: false,
                                ignore_if_not_exists: false,
                            },
                        ));
                    }
                    Some(ChangeJsonValue::Remove) => patches.push((
                        PatchOperation::Remove(RemoveOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "{body_block}/urlencoded/{id}/value"
                                ))
                            },
                        }),
                        EditOptions {
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    )),
                    _ => {}
                }

                match &urlencoded_param_to_update.description {
                    Some(ChangeString::Update(value)) => {
                        patches.push((
                            PatchOperation::Replace(ReplaceOperation {
                                path: unsafe {
                                    PointerBuf::new_unchecked(format!(
                                        "{body_block}/urlencoded/{id}/description"
                                    ))
                                },
                                value: JsonValue::String(value.clone()),
                            }),
                            EditOptions {
                                create_missing_segments: false,
                                ignore_if_not_exists: false,
                            },
                        ));
                    }
                    Some(ChangeString::Remove) => {
                        patches.push((
                            PatchOperation::Remove(RemoveOperation {
                                path: unsafe {
                                    PointerBuf::new_unchecked(format!(
                                        "{body_block}/urlencoded/{id}/description"
                                    ))
                                },
                            }),
                            EditOptions {
                                create_missing_segments: false,
                                ignore_if_not_exists: false,
                            },
                        ));
                    }
                    _ => {}
                }

                if let Some(options) = &urlencoded_param_to_update.options {
                    let options = UrlencodedParamSpecOptions {
                        disabled: options.disabled,
                        propagate: options.propagate,
                    };
                    let options_value = continue_if_err!(serde_json::to_value(options), |err| {
                        session::error!(format!("failed to convert options value: {}", err))
                    });

                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "{body_block}/urlencoded/{id}/options"
                                ))
                            },
                            value: options_value,
                        }),
                        EditOptions {
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }

                if let Some(order) = urlencoded_param_to_update.order {
                    param_order_updates.insert(id, order);
                }
            }

            for id in params_to_remove {
                patches.push((
                    PatchOperation::Remove(RemoveOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!("{body_block}/urlencoded/{id}"))
                        },
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
                param_order_removes.push(id);
            }

            // We don't want database failures to stop the operation
            let mut txn = if let Ok(txn) = worktree.storage.begin_write(ctx).await {
                txn
            } else {
                return Ok(Some(BodyKind::Urlencoded));
            };

            for (id, order) in param_order_updates {
                if let Err(e) = worktree
                    .storage
                    .put_entry_body_urlencoded_param_order_txn(ctx, &mut txn, &entry_id, &id, order)
                    .await
                {
                    session::error!(format!("failed to update urlencoded param order: {}", e));
                    return Ok(Some(BodyKind::Urlencoded));
                }
            }

            for id in param_order_removes {
                if let Err(e) = worktree
                    .storage
                    .remove_entry_body_urlencoded_param_order_txn(ctx, &mut txn, &entry_id, &id)
                    .await
                {
                    session::error!(format!("failed to remove urlencoded param order: {}", e));
                    return Ok(Some(BodyKind::Urlencoded));
                }
            }

            if let Err(e) = txn.commit() {
                session::error!(format!("failed to commit transaction: {}", e));
                return Ok(Some(BodyKind::Urlencoded));
            }
            Some(BodyKind::Urlencoded)
        }
        UpdateBodyParams::FormData {
            params_to_add,
            params_to_update,
            params_to_remove,
        } => {
            let body_block = "/body/form-data";
            if current_body_kind.is_some() && current_body_kind != Some(BodyKind::FormData) {
                clear_item_body(worktree, ctx, entry_id.clone(), patches).await?;
                // Create the body block even if no parameter is given
                patches.push((
                    PatchOperation::Add(AddOperation {
                        path: unsafe { PointerBuf::new_unchecked("/body") },
                        value: json!({
                            "form-data": {}
                        }),
                    }),
                    EditOptions {
                        create_missing_segments: true,
                        ignore_if_not_exists: false,
                    },
                ));
            }

            let mut param_order_updates = HashMap::new();
            let mut param_order_removes = Vec::new();

            for formdata_param_to_add in params_to_add {
                let id = formdata_param_to_add
                    .id
                    .clone()
                    .unwrap_or(FormDataParamId::new());
                let id_str = id.to_string();

                let value = continue_if_err!(json_to_hcl(&formdata_param_to_add.value), |err| {
                    session::error!(format!("failed to convert value expression: {}", err));
                    let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                        activity_id: "expression_conversion_error",
                        title: "Failed to convert value expression".to_string(),
                        detail: Some(err),
                    });
                });

                let spec = FormDataParamSpec {
                    name: formdata_param_to_add.name.clone(),
                    value,
                    description: formdata_param_to_add.description.clone(),
                    options: FormDataParamSpecOptions {
                        disabled: formdata_param_to_add.options.disabled,
                        propagate: formdata_param_to_add.options.propagate,
                    },
                };

                let spec_value = continue_if_err!(
                    serde_json::to_value(&spec).map_err(|err| err.to_string()),
                    |err| {
                        session::error!(format!(
                            "failed to convert formdata param spec to json: {}",
                            err
                        ));
                        let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                            activity_id: "formdata_param_spec_conversion_error",
                            title: "Failed to convert formdata param spec to json".to_string(),
                            detail: Some(err),
                        });
                    }
                );

                patches.push((
                    PatchOperation::Add(AddOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!("{body_block}/form_data/{id_str}"))
                        },
                        value: spec_value,
                    }),
                    EditOptions {
                        create_missing_segments: true,
                        ignore_if_not_exists: false,
                    },
                ));
                param_order_updates.insert(id, formdata_param_to_add.order);
            }

            for formdata_param_to_update in params_to_update {
                let id = formdata_param_to_update.id.clone();
                if let Some(new_name) = &formdata_param_to_update.name {
                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "{body_block}/form_data/{id}/name"
                                ))
                            },
                            value: JsonValue::String(new_name.clone()),
                        }),
                        EditOptions {
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }

                match &formdata_param_to_update.value {
                    Some(ChangeJsonValue::Update(value)) => {
                        patches.push((
                            PatchOperation::Replace(ReplaceOperation {
                                path: unsafe {
                                    PointerBuf::new_unchecked(format!(
                                        "{body_block}/form_data/{id}/value"
                                    ))
                                },
                                value: value.clone(),
                            }),
                            EditOptions {
                                create_missing_segments: false,
                                ignore_if_not_exists: false,
                            },
                        ));
                    }
                    Some(ChangeJsonValue::Remove) => patches.push((
                        PatchOperation::Remove(RemoveOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "{body_block}/form_data/{id}/value"
                                ))
                            },
                        }),
                        EditOptions {
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    )),
                    _ => {}
                }

                match &formdata_param_to_update.description {
                    Some(ChangeString::Update(value)) => {
                        patches.push((
                            PatchOperation::Replace(ReplaceOperation {
                                path: unsafe {
                                    PointerBuf::new_unchecked(format!(
                                        "{body_block}/form_data/{id}/description"
                                    ))
                                },
                                value: JsonValue::String(value.clone()),
                            }),
                            EditOptions {
                                create_missing_segments: false,
                                ignore_if_not_exists: false,
                            },
                        ));
                    }
                    Some(ChangeString::Remove) => {
                        patches.push((
                            PatchOperation::Remove(RemoveOperation {
                                path: unsafe {
                                    PointerBuf::new_unchecked(format!(
                                        "{body_block}/form_data/{id}/description"
                                    ))
                                },
                            }),
                            EditOptions {
                                create_missing_segments: false,
                                ignore_if_not_exists: false,
                            },
                        ));
                    }
                    _ => {}
                }

                if let Some(options) = &formdata_param_to_update.options {
                    let options = FormDataParamSpecOptions {
                        disabled: options.disabled,
                        propagate: options.propagate,
                    };
                    let options_value = continue_if_err!(serde_json::to_value(options), |err| {
                        session::error!(format!("failed to convert options value: {}", err))
                    });

                    patches.push((
                        PatchOperation::Replace(ReplaceOperation {
                            path: unsafe {
                                PointerBuf::new_unchecked(format!(
                                    "{body_block}/form_data/{id}/options"
                                ))
                            },
                            value: options_value,
                        }),
                        EditOptions {
                            create_missing_segments: false,
                            ignore_if_not_exists: false,
                        },
                    ));
                }

                if let Some(order) = formdata_param_to_update.order {
                    param_order_updates.insert(id, order);
                }
            }

            for id in params_to_remove {
                patches.push((
                    PatchOperation::Remove(RemoveOperation {
                        path: unsafe {
                            PointerBuf::new_unchecked(format!("{body_block}/form_data/{id}"))
                        },
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
                param_order_removes.push(id);
            }

            // We don't want database failures to stop the operation
            let mut txn = if let Ok(txn) = worktree.storage.begin_write(ctx).await {
                txn
            } else {
                return Ok(Some(BodyKind::FormData));
            };

            for (id, order) in param_order_updates {
                if let Err(e) = worktree
                    .storage
                    .put_entry_body_formdata_param_order_txn(ctx, &mut txn, &entry_id, &id, order)
                    .await
                {
                    session::error!(format!("failed to update formdata param order: {}", e));
                    return Ok(Some(BodyKind::FormData));
                }
            }

            for id in param_order_removes {
                if let Err(e) = worktree
                    .storage
                    .remove_entry_body_formdata_param_order_txn(ctx, &mut txn, &entry_id, &id)
                    .await
                {
                    session::error!(format!("failed to remove formdata param order: {}", e));
                    return Ok(Some(BodyKind::FormData));
                }
            }

            if let Err(e) = txn.commit() {
                session::error!(format!("failed to commit transaction: {}", e));
                return Ok(Some(BodyKind::FormData));
            }
            Some(BodyKind::FormData)
        }
    };

    Ok(new_body_kind)
}

async fn clear_item_body<R: AppRuntime>(
    worktree: &Worktree<R>,
    ctx: &R::AsyncContext,
    id: ResourceId,
    patches: &mut Vec<(PatchOperation, EditOptions)>,
) -> joinerror::Result<()> {
    worktree.storage.remove_entry_body_cache(ctx, &id).await?;
    patches.push((
        PatchOperation::Remove(RemoveOperation {
            path: unsafe { PointerBuf::new_unchecked("/body") },
        }),
        EditOptions {
            create_missing_segments: false,
            ignore_if_not_exists: true,
        },
    ));
    Ok(())
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
    expanded_entries: &HashSet<ResourceId>,
    fs: &Arc<dyn FileSystem>,
    abs_path: &Path,
) -> joinerror::Result<Option<(Entry, EntryDescription)>> {
    let dir_config_path = abs_path.join(constants::DIR_CONFIG_FILENAME);
    let item_config_path = abs_path.join(constants::ITEM_CONFIG_FILENAME);

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());

    if fs.is_dir_empty(&abs_path).await? {
        session::info!(format!(
            "Deleting empty entry folder: {}",
            abs_path.display()
        ));
        fs.remove_dir(
            &abs_path,
            RemoveOptions {
                recursive: false,
                ignore_if_not_exists: false,
            },
        )
        .await?;
        return Ok(None);
    }

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
            kind: ResourceKind::Dir,
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
                edit: EntryEditing::new(fs.clone(), path_tx, DIR_CONFIG_FILENAME),
                class: model.class(),
                protocol: None,
                metadata: EntryMetadata { body_kind: None },
            },
            desc,
        )));
    } else if item_config_path.exists() {
        let mut rdr = fs.open_file(&item_config_path).await?;
        let model: EntryModel =
            hcl::from_reader(&mut rdr).join_err::<()>("failed to parse item configuration")?;

        let id = model.id().clone();
        let desc = EntryDescription {
            id: id.clone(),
            name: desanitize(&name),
            path: path.clone(),
            class: model.class(),
            kind: ResourceKind::Item,
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
                edit: EntryEditing::new(fs.clone(), path_tx, ITEM_CONFIG_FILENAME),
                class: model.class(),
                protocol: model.protocol(),
                metadata: EntryMetadata {
                    body_kind: model.body_kind(),
                },
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

async fn describe_body<R: AppRuntime>(
    app_delegate: &AppDelegate<R>,
    entry_id: &ResourceId,
    body: LabeledBlock<IndexMap<BodyKind, BodySpec>>,
    entry_keys: &HashMap<SegKeyBuf, AnyValue>,
) -> Option<BodyInfo> {
    let (kind, spec) = body
        .iter()
        .map(|(kind, spec)| (kind, spec.clone()))
        .next()?;

    let inner = match kind {
        BodyKind::Text => BodyInfo::Text(spec.text?),
        BodyKind::Json => BodyInfo::Json(spec.json?),
        BodyKind::Xml => BodyInfo::Xml(spec.xml?),
        BodyKind::Binary => BodyInfo::Binary(spec.binary?),
        BodyKind::Urlencoded => {
            if spec.urlencoded.is_none() {
                return Some(BodyInfo::Urlencoded(vec![]));
            }
            let mut param_infos = Vec::new();
            for (param_id, param_spec) in spec.urlencoded?.into_inner() {
                let value = match hcl_to_json(&param_spec.value) {
                    Ok(value) => value,
                    Err(err) => {
                        session::error!(format!(
                            "failed to convert value expression `{}`: {}",
                            &param_spec.value,
                            err.to_string()
                        ));
                        let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                            activity_id: "expression_conversion_error",
                            title: "Failed to convert value expression".to_string(),
                            detail: Some(err.to_string()),
                        });
                        JsonValue::Null
                    }
                };

                param_infos.push(UrlencodedParamInfo {
                    id: param_id.clone(),
                    name: param_spec.name,
                    value,
                    description: param_spec.description,
                    disabled: param_spec.options.disabled,
                    propagate: param_spec.options.propagate,
                    order: entry_keys
                        .get(&segkey_entry_body_urlencoded_param_order(
                            entry_id, &param_id,
                        ))
                        .and_then(|value| value.deserialize().ok()),
                });
            }
            BodyInfo::Urlencoded(param_infos)
        }
        BodyKind::FormData => {
            if spec.form_data.is_none() {
                return Some(BodyInfo::FormData(vec![]));
            }

            let mut param_infos = Vec::new();
            for (param_id, param_spec) in spec.form_data?.into_inner() {
                let value = match hcl_to_json(&param_spec.value) {
                    Ok(value) => value,
                    Err(err) => {
                        session::error!(format!(
                            "failed to convert value expression `{}`: {}",
                            &param_spec.value,
                            err.to_string()
                        ));
                        let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                            activity_id: "expression_conversion_error",
                            title: "Failed to convert value expression".to_string(),
                            detail: Some(err.to_string()),
                        });
                        JsonValue::Null
                    }
                };

                param_infos.push(FormDataParamInfo {
                    id: param_id.clone(),
                    name: param_spec.name,
                    value,
                    description: param_spec.description,
                    disabled: param_spec.options.disabled,
                    propagate: param_spec.options.propagate,
                    order: entry_keys
                        .get(&segkey_entry_body_formdata_param_order(entry_id, &param_id))
                        .and_then(|value| value.deserialize().ok()),
                });
            }
            BodyInfo::FormData(param_infos)
        }
    };
    Some(inner)
}
