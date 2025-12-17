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
use moss_edit::json::EditOptions;
use moss_fs::{CreateOptions, FileSystem, RemoveOptions, desanitize_path, utils::SanitizedPath};
use moss_hcl::{HclResultExt, hcl_to_json, json_to_hcl};
use moss_logging::session;
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use moss_text::sanitized::{desanitize, sanitize};

use sapic_base::{
    language::i18n::NO_TRANSLATE_KEY, localize, project::types::primitives::ProjectId,
    resource::types::primitives::ResourceId,
};
use sapic_core::context::AnyAsyncContext;
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
            FormDataParamId, HeaderId, PathParamId, QueryParamId, ResourceKind, ResourceProtocol,
            UrlencodedParamId,
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
        KEY_EXPANDED_ENTRIES, key_resource, key_resource_body, key_resource_body_formdata_param,
        key_resource_body_formdata_param_order, key_resource_body_urlencoded_param,
        key_resource_body_urlencoded_param_order, key_resource_header, key_resource_header_order,
        key_resource_order, key_resource_path_param, key_resource_path_param_order,
        key_resource_query_param, key_resource_query_param_order,
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

pub(crate) struct Worktree {
    storage: Arc<dyn KvStorage>,
    // Used for storage scope
    project_id: ProjectId,
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    state: Arc<RwLock<WorktreeState>>,
}

// Required for OnceCell::set
impl Debug for Worktree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Worktree")
            .field("abs_path", &self.abs_path)
            .finish()
    }
}

impl Worktree {
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
        ctx: &dyn AnyAsyncContext,
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
                ctx,
                &abs_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        if let Err(e) = self
            .storage
            .remove_batch_by_prefix(
                ctx,
                StorageScope::Project(self.project_id.inner()),
                &key_resource(id),
            )
            .await
        {
            session::warn!(format!(
                "failed to update database after removing resource entry: {}",
                e
            ));
        }

        let update_expanded_entries = state_lock.expanded_entries.remove(&id);
        if !update_expanded_entries {
            return Ok(());
        }

        if let Err(e) = self
            .storage
            .put(
                ctx,
                StorageScope::Project(self.project_id.inner()),
                KEY_EXPANDED_ENTRIES,
                serde_json::to_value(&state_lock.expanded_entries)?,
            )
            .await
        {
            session::warn!(format!(
                "failed to update extended_entries after removing resource entry: {} ",
                e
            ));
        }

        Ok(())
    }

    pub async fn scan<R: AppRuntime>(
        &self,
        ctx: Arc<dyn AnyAsyncContext>, // TODO: use ctx ctx.done() to cancel the scan if needed
        app_delegate: AppDelegate<R>,
        path: &Path,
        expanded_entries: Arc<HashSet<ResourceId>>,
        all_entry_keys: Arc<HashMap<String, JsonValue>>,
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
            title: localize!("workbench.activity.scanning", "Scanning"),
            detail: None,
        })?;

        let mut handles = Vec::new();
        while let Some(job) = job_rx.recv().await {
            let sender = sender.clone();
            let fs = self.fs.clone();
            let state = self.state.clone();
            let app_delegate = app_delegate.clone();
            let expanded_entries = expanded_entries.clone();
            let all_entry_keys = all_entry_keys.clone();

            activity_handle.emit_progress(Some(localize!(
                NO_TRANSLATE_KEY,
                job.path.display().to_string()
            )))?;

            let ctx_clone = ctx.clone();
            let handle = tokio::spawn(async move {
                let mut new_jobs = Vec::new();
                let ctx_clone = ctx_clone.clone();
                if !job.path.as_os_str().is_empty() {
                    match process_entry(
                        ctx_clone.clone(),
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

                            if let Err(e) = sender.send(desc) {
                                session::debug!(format!(
                                    "failed to send EntryDescription to tokio mpsc channel: {}",
                                    e
                                ));
                            }

                            state.write().await.entries.insert(entry.id.clone(), entry);
                        }
                        Ok(None) => {
                            session::info!(format!(
                                "encountered an empty entry dir: {}",
                                job.abs_path.display()
                            ));
                            return;
                        }
                        Err(err) => {
                            session::error!(format!(
                                "error processing dir: {}",
                                job.abs_path.display()
                            ));
                            let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                                activity_id: "worktree_scan_process_entry_error",
                                title: localize!(
                                    "workbench.activity.error_processing_dir",
                                    "Error processing dir"
                                ),
                                detail: Some(localize!(
                                    "workbench.activity.error_processing_dir_detail",
                                    format!(
                                        "Error processing dir {}: {}",
                                        job.abs_path.display(),
                                        err
                                    )
                                )),
                            });
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
                                ctx_clone.clone(),
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
                        session::warn!(format!(
                            "non-entry encountered during scan: {}",
                            child_abs_path.display()
                        ));
                    });

                    // INFO: Something here doesn't feel quite right—maybe we can improve it once we have the UI
                    if child_file_type.is_dir() {
                        new_jobs.push(ScanJob {
                            abs_path: Arc::clone(&child_abs_path),
                            path: child_path,
                            scan_queue: job.scan_queue.clone(),
                        });
                    } else {
                        if let Err(e) = sender.send(desc) {
                            session::debug!(format!(
                                "failed to send EntryDescription to tokio mpsc channel: {}",
                                e
                            ));
                        }
                    }

                    state.write().await.entries.insert(entry.id.clone(), entry);
                }

                for new_job in new_jobs {
                    if let Err(e) = job.scan_queue.send(new_job) {
                        session::debug!(format!(
                            "failed to send ScanJob to tokio mpsc channel: {}",
                            e
                        ));
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Err(err) = handle.await {
                session::error!(format!("error joining job: {}", err));
            }
        }

        activity_handle.emit_finish()?;

        Ok(())
    }

    pub async fn create_item_entry(
        &self,
        ctx: &dyn AnyAsyncContext,
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
        self.create_entry_internal(ctx, &sanitized_path, false, &content.as_bytes())
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

        let order_key = key_resource_order(&id);
        let mut batch_input = vec![(order_key.as_str(), serde_json::to_value(order)?)];

        if expanded {
            state_lock.expanded_entries.insert(id);
            batch_input.push((
                KEY_EXPANDED_ENTRIES,
                serde_json::to_value(&state_lock.expanded_entries)?,
            ));
        }

        if let Err(e) = self
            .storage
            .put_batch(
                ctx,
                StorageScope::Project(self.project_id.inner()),
                &batch_input,
            )
            .await
        {
            session::warn!(format!(
                "failed to update database after creating item entry: {}",
                e
            ));
        }

        Ok(())
    }

    pub async fn create_dir_entry(
        &self,
        ctx: &dyn AnyAsyncContext,
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
        self.create_entry_internal(ctx, &sanitized_path, true, &content.as_bytes())
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

        let order_key = key_resource_order(&id);

        let mut batch_input = vec![(order_key.as_str(), serde_json::to_value(order)?)];

        if expanded {
            state_lock.expanded_entries.insert(id);
            batch_input.push((
                KEY_EXPANDED_ENTRIES,
                serde_json::to_value(&state_lock.expanded_entries)?,
            ));
        }

        if let Err(e) = self
            .storage
            .put_batch(
                ctx,
                StorageScope::Project(self.project_id.inner()),
                &batch_input,
            )
            .await
        {
            session::warn!(format!(
                "failed to update database after creating dir entry: {}",
                e
            ));
        }

        Ok(())
    }

    pub async fn update_dir_entry(
        &self,
        ctx: &dyn AnyAsyncContext,
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
                    ctx,
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
                    ctx,
                    &self.abs_path.join(dirs::RESOURCES_DIR),
                    &old_path,
                    &new_path,
                )
                .await?;
        }

        // TODO: patch the dir entry

        let path = entry.path_rx.borrow().clone();

        let mut batch_input = vec![];
        let order_key = key_resource_order(&id);

        if let Some(order) = params.order {
            batch_input.push((order_key.as_str(), serde_json::to_value(order)?));
        }

        if let Some(expanded) = params.expanded {
            if expanded {
                state_lock.expanded_entries.insert(id.to_owned());
            } else {
                state_lock.expanded_entries.remove(id);
            }
            batch_input.push((
                KEY_EXPANDED_ENTRIES,
                serde_json::to_value(&state_lock.expanded_entries)?,
            ));
        }

        if batch_input.is_empty() {
            return Ok(path);
        }

        if let Err(e) = self
            .storage
            .put_batch(
                ctx,
                StorageScope::Project(self.project_id.inner()),
                &batch_input,
            )
            .await
        {
            session::warn!(format!(
                "failed to update database after updating dir entry: {}",
                e
            ));
        }

        Ok(path)
    }

    pub async fn update_item_entry<R: AppRuntime>(
        &self,
        ctx: &dyn AnyAsyncContext,
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
                    ctx,
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
                    ctx,
                    &self.abs_path.join(dirs::RESOURCES_DIR),
                    &old_path,
                    &new_path,
                )
                .await?;
        }

        self.patch_item_entry(ctx, app_delegate, entry, &params)
            .await?;

        let path = entry.path_rx.borrow().clone();

        let mut batch_input = vec![];
        let order_key = key_resource_order(&id);

        if let Some(order) = params.order {
            batch_input.push((order_key.as_str(), serde_json::to_value(order)?));
        }

        if let Some(expanded) = params.expanded {
            if expanded {
                state_lock.expanded_entries.insert(id.to_owned());
            } else {
                state_lock.expanded_entries.remove(id);
            }
            batch_input.push((
                KEY_EXPANDED_ENTRIES,
                serde_json::to_value(&state_lock.expanded_entries)?,
            ));
        }

        if batch_input.is_empty() {
            return Ok(path);
        }

        if let Err(e) = self
            .storage
            .put_batch(
                ctx,
                StorageScope::Project(self.project_id.inner()),
                &batch_input,
            )
            .await
        {
            session::warn!(format!(
                "failed to update database after updating dir entry: {}",
                e
            ));
        }

        Ok(path)
    }

    pub async fn describe_entry<R: AppRuntime>(
        &self,
        ctx: &dyn AnyAsyncContext,
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
            let mut rdr = self.fs.open_file(ctx, &dir_config_path).await?;
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
                .get_batch_by_prefix(
                    ctx,
                    StorageScope::Project(self.project_id.inner()),
                    &key_resource(&id),
                )
                .await
                .unwrap_or_else(|e| {
                    session::error!(format!("failed to get entry cache: {}", e));
                    Vec::new()
                })
                .into_iter()
                .collect::<HashMap<_, _>>();

            let mut rdr = self.fs.open_file(ctx, &item_config_path).await?;
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
                                title: localize!(
                                    "workbench.activity.failed_to_convert_value_expression",
                                    "Failed to convert value expression"
                                ),
                                detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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
                            .get(&key_resource_header_order(id, &header_id))
                            .and_then(|value| serde_json::from_value(value.clone()).ok()),
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
                                title: localize!(
                                    "workbench.activity.failed_to_convert_value_expression",
                                    "Failed to convert value expression"
                                ),
                                detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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
                            .get(&key_resource_path_param_order(id, &path_param_id))
                            .and_then(|value| serde_json::from_value(value.clone()).ok()),
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
                                title: localize!(
                                    "workbench.activity.failed_to_convert_value_expression",
                                    "Failed to convert value expression"
                                ),
                                detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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
                            .get(&key_resource_query_param_order(id, &query_param_id))
                            .and_then(|value| serde_json::from_value(value.clone()).ok()),
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

impl Worktree {
    pub fn new(
        storage: Arc<dyn KvStorage>,
        project_id: ProjectId,
        abs_path: Arc<Path>,
        fs: Arc<dyn FileSystem>,
    ) -> Self {
        Self {
            storage,
            project_id,
            abs_path,
            fs,
            state: Default::default(),
        }
    }
}

impl Worktree {
    async fn create_entry_internal(
        &self,
        ctx: &dyn AnyAsyncContext,
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

        self.fs.create_dir(ctx, &abs_path).await?;

        let file_path = if is_dir {
            abs_path.join(constants::DIR_CONFIG_FILENAME)
        } else {
            abs_path.join(constants::ITEM_CONFIG_FILENAME)
        };

        self.fs
            .create_file_with(
                ctx,
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

    async fn patch_item_entry<R: AppRuntime>(
        &self,
        ctx: &dyn AnyAsyncContext,
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

        let storage_scope = StorageScope::Project(self.project_id.inner());

        for header_to_add in &params.headers_to_add {
            let id = HeaderId::new();
            let id_str = id.to_string();

            let value = continue_if_err!(json_to_hcl(&header_to_add.value), |err: String| {
                session::error!(format!("failed to convert value expression: {}", err));
                let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                    activity_id: "expression_conversion_error",
                    title: localize!(
                        "workbench.activity.failed_to_convert_value_expression",
                        "Failed to convert value expression"
                    ),
                    detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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
                |err: String| {
                    session::error!(format!("failed to convert header spec to json: {}", err));
                    let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                        activity_id: "header_spec_conversion_error",
                        title: localize!(
                            "workbench.activity.failed_to_convert_header_spec_to_json",
                            "Failed to convert header spec to json"
                        ),
                        detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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

            if let Err(e) = self
                .storage
                .put(
                    ctx,
                    storage_scope.clone(),
                    &key_resource_header_order(&entry.id, &id),
                    serde_json::to_value(&header_to_add.order)?,
                )
                .await
            {
                session::warn!(format!("failed to put header order in the database: {}", e));
            }
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
                if let Err(e) = self
                    .storage
                    .put(
                        ctx,
                        storage_scope.clone(),
                        &key_resource_header_order(&entry.id, &header_to_update.id),
                        serde_json::to_value(&order)?,
                    )
                    .await
                {
                    session::warn!(format!("failed to put header order in the database: {}", e));
                }
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

            if let Err(e) = self
                .storage
                .remove_batch_by_prefix(
                    ctx,
                    storage_scope.clone(),
                    &key_resource_header(&entry.id, id),
                )
                .await
            {
                session::warn!(format!("failed to remove header cache: {}", e));
            }
        }

        for path_param_to_add in &params.path_params_to_add {
            let id = PathParamId::new();
            let id_str = id.to_string();

            let value = continue_if_err!(json_to_hcl(&path_param_to_add.value), |err: String| {
                session::error!("failed to convert value expression: {}", err);
                let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                    activity_id: "expression_conversion_error",
                    title: localize!(
                        "workbench.activity.failed_to_convert_value_expression",
                        "Failed to convert value expression"
                    ),
                    detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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
                |err: String| {
                    session::error!(format!(
                        "failed to convert path param spec to json: {}",
                        err
                    ));
                    let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                        activity_id: "expression_conversion_error",
                        title: localize!(
                            "workbench.activity.failed_to_convert_value_expression",
                            "Failed to convert value expression"
                        ),
                        detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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

            if let Err(e) = self
                .storage
                .put(
                    ctx,
                    storage_scope.clone(),
                    &key_resource_path_param_order(&entry.id, &id),
                    serde_json::to_value(&path_param_to_add.order)?,
                )
                .await
            {
                session::warn!(format!(
                    "failed to put path param order in the database: {}",
                    e
                ));
            }
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
                if let Err(e) = self
                    .storage
                    .put(
                        ctx,
                        storage_scope.clone(),
                        &key_resource_path_param_order(&entry.id, &path_param_to_update.id),
                        serde_json::to_value(&order)?,
                    )
                    .await
                {
                    session::warn!(format!(
                        "failed to put path param order in the database: {}",
                        e
                    ));
                }
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

            if let Err(e) = self
                .storage
                .remove_batch_by_prefix(
                    ctx,
                    storage_scope.clone(),
                    &key_resource_path_param(&entry.id, id),
                )
                .await
            {
                session::warn!(format!("failed to remove path param cache: {}", e));
            }
        }

        for query_param_to_add in &params.query_params_to_add {
            let id = QueryParamId::new();
            let id_str = id.to_string();

            let value = continue_if_err!(json_to_hcl(&query_param_to_add.value), |err: String| {
                session::error!("failed to convert value expression: {}", err);
                let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                    activity_id: "expression_conversion_error",
                    title: localize!(
                        "workbench.activity.failed_to_convert_value_expression",
                        "Failed to convert value expression"
                    ),
                    detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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
                |err: String| {
                    session::error!(format!(
                        "failed to convert query param spec to json: {}",
                        err
                    ));
                    let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                        activity_id: "query_param_spec_conversion_error",
                        title: localize!(
                            "workbench.activity.failed_to_convert_query_param_spec_to_json",
                            "Failed to convert query param spec to json"
                        ),
                        detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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

            if let Err(e) = self
                .storage
                .put(
                    ctx,
                    storage_scope.clone(),
                    &key_resource_query_param_order(&entry.id, &id),
                    serde_json::to_value(&query_param_to_add.order)?,
                )
                .await
            {
                session::warn!(format!(
                    "failed to put query param order in the database: {}",
                    e
                ));
            }
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
                if let Err(e) = self
                    .storage
                    .put(
                        ctx,
                        storage_scope.clone(),
                        &key_resource_query_param_order(&entry.id, &query_param_to_update.id),
                        serde_json::to_value(&order)?,
                    )
                    .await
                {
                    session::warn!(format!(
                        "failed to put query param order in the database: {}",
                        e
                    ));
                }
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

            if let Err(e) = self
                .storage
                .remove_batch_by_prefix(
                    ctx,
                    storage_scope.clone(),
                    &key_resource_query_param(&entry.id, id),
                )
                .await
            {
                session::warn!(format!("failed to remove query param cache: {}", e));
            }
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
            .edit(ctx, &self.abs_path.join(dirs::RESOURCES_DIR), &patches)
            .await?;

        for callback in on_edit_success {
            callback(entry);
        }

        Ok(())
    }
}

async fn patch_item_body<R: AppRuntime>(
    worktree: &Worktree,
    ctx: &dyn AnyAsyncContext,
    app_delegate: &AppDelegate<R>,
    current_body_kind: Option<BodyKind>,
    resource_id: ResourceId,
    patches: &mut Vec<(PatchOperation, EditOptions)>,
    params: &UpdateBodyParams,
) -> joinerror::Result<Option<BodyKind>> {
    let storage = worktree.storage.clone();
    let storage_scope = StorageScope::Project(worktree.project_id.inner());
    let new_body_kind = match params {
        UpdateBodyParams::Remove => {
            if let Err(e) = clear_item_body(worktree, ctx, &resource_id, storage, patches).await {
                session::warn!(format!("failed to clear old item body cache: {}", e));
            }
            None
        }
        UpdateBodyParams::Text(text) => {
            if current_body_kind.is_some() && current_body_kind != Some(BodyKind::Text) {
                if let Err(e) = clear_item_body(worktree, ctx, &resource_id, storage, patches).await
                {
                    session::warn!(format!("failed to clear old item body cache: {}", e));
                }
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
                if let Err(e) = clear_item_body(worktree, ctx, &resource_id, storage, patches).await
                {
                    session::warn!(format!("failed to clear old item body cache: {}", e));
                }
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
                if let Err(e) = clear_item_body(worktree, ctx, &resource_id, storage, patches).await
                {
                    session::warn!(format!("failed to clear old item body cache: {}", e));
                }
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
                if let Err(e) = clear_item_body(worktree, ctx, &resource_id, storage, patches).await
                {
                    session::warn!(format!("failed to clear old item body cache: {}", e));
                }
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
                if let Err(e) =
                    clear_item_body(worktree, ctx, &resource_id, storage.clone(), patches).await
                {
                    session::warn!(format!("failed to clear old item body cache: {}", e));
                }
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
            let mut param_removes = Vec::new();

            for urlencoded_param_to_add in params_to_add {
                let id = urlencoded_param_to_add
                    .id
                    .clone()
                    .unwrap_or(UrlencodedParamId::new());
                let id_str = id.to_string();

                let value = continue_if_err!(
                    json_to_hcl(&urlencoded_param_to_add.value),
                    |err: String| {
                        session::error!(format!("failed to convert value expression: {}", err));
                        let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                            activity_id: "expression_conversion_error",
                            title: localize!(
                                "workbench.activity.failed_to_convert_value_expression",
                                "Failed to convert value expression"
                            ),
                            detail: Some(localize!(NO_TRANSLATE_KEY, err)),
                        });
                    }
                );

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
                    |err: String| {
                        session::error!(format!(
                            "failed to convert urlencoded param spec to json: {}",
                            err
                        ));
                        let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                        activity_id: "urlencoded_param_spec_conversion_error",
                        title: localize!(
                            "workbench.activity.failed_to_convert_urlencoded_param_spec_to_json",
                            "Failed to convert urlencoded param spec to json"
                        ),
                        detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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
                param_removes.push(id);
            }

            for (id, order) in param_order_updates {
                if let Err(e) = storage
                    .put(
                        ctx,
                        storage_scope.clone(),
                        &key_resource_body_urlencoded_param_order(&resource_id, &id),
                        serde_json::to_value(order)?,
                    )
                    .await
                {
                    session::warn!(format!("failed to update urlencoded param order: {}", e));
                }
            }

            for id in param_removes {
                if let Err(e) = storage
                    .remove(
                        ctx,
                        storage_scope.clone(),
                        &key_resource_body_urlencoded_param(&resource_id, &id),
                    )
                    .await
                {
                    session::warn!(format!("failed to remove urlencoded param cache: {}", e));
                }
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
                if let Err(e) =
                    clear_item_body(worktree, ctx, &resource_id, storage.clone(), patches).await
                {
                    session::warn!(format!("failed to clear old item body cache: {}", e));
                }
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
            let mut param_removes = Vec::new();

            for formdata_param_to_add in params_to_add {
                let id = formdata_param_to_add
                    .id
                    .clone()
                    .unwrap_or(FormDataParamId::new());
                let id_str = id.to_string();

                let value =
                    continue_if_err!(json_to_hcl(&formdata_param_to_add.value), |err: String| {
                        session::error!(format!("failed to convert value expression: {}", err));
                        let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                            activity_id: "expression_conversion_error",
                            title: localize!(
                                "workbench.activity.failed_to_convert_value_expression",
                                "Failed to convert value expression"
                            ),
                            detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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
                    |err: String| {
                        session::error!(format!(
                            "failed to convert formdata param spec to json: {}",
                            err
                        ));
                        let _ = app_delegate.emit_oneshot(ToLocation::Toast {
                            activity_id: "formdata_param_spec_conversion_error",
                            title: localize!(
                                "workbench.activity.failed_to_convert_formdata_param_spec_to_json",
                                "Failed to convert formdata param spec to json"
                            ),
                            detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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
                param_removes.push(id);
            }

            for (id, order) in param_order_updates {
                if let Err(e) = storage
                    .put(
                        ctx,
                        storage_scope.clone(),
                        &key_resource_body_formdata_param_order(&resource_id, &id),
                        serde_json::to_value(order)?,
                    )
                    .await
                {
                    session::warn!(format!("failed to update formdata param order: {}", e));
                }
            }

            for id in param_removes {
                if let Err(e) = storage
                    .remove(
                        ctx,
                        storage_scope.clone(),
                        &key_resource_body_formdata_param(&resource_id, &id),
                    )
                    .await
                {
                    session::warn!(format!("failed to remove formdata param cache: {}", e));
                }
            }

            Some(BodyKind::FormData)
        }
    };

    Ok(new_body_kind)
}

async fn clear_item_body(
    worktree: &Worktree,
    ctx: &dyn AnyAsyncContext,
    id: &ResourceId,
    storage: Arc<dyn KvStorage>,
    patches: &mut Vec<(PatchOperation, EditOptions)>,
) -> joinerror::Result<()> {
    patches.push((
        PatchOperation::Remove(RemoveOperation {
            path: unsafe { PointerBuf::new_unchecked("/body") },
        }),
        EditOptions {
            create_missing_segments: false,
            ignore_if_not_exists: true,
        },
    ));

    if let Err(e) = storage
        .remove_batch_by_prefix(
            ctx,
            StorageScope::Project(worktree.project_id.inner()),
            &key_resource_body(id),
        )
        .await
    {
        return Err(e);
    }

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
    ctx: Arc<dyn AnyAsyncContext>,
    path: Arc<Path>,
    all_entry_keys: &HashMap<String, JsonValue>,
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

    if fs.is_dir_empty(ctx.as_ref(), &abs_path).await? {
        session::info!(format!(
            "Deleting empty entry folder: {}",
            abs_path.display()
        ));
        fs.remove_dir(
            ctx.as_ref(),
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
        let mut rdr = fs.open_file(ctx.as_ref(), &dir_config_path).await?;
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
                .get(&key_resource_order(&id))
                .and_then(|value| serde_json::from_value(value.clone()).ok()),
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
        let mut rdr = fs.open_file(ctx.as_ref(), &item_config_path).await?;
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
                .get(&key_resource_order(&id))
                .and_then(|value| serde_json::from_value(value.clone()).ok()),
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
    entry_keys: &HashMap<String, JsonValue>,
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
                            title: localize!(
                                "workbench.activity.failed_to_convert_value_expression",
                                "Failed to convert value expression"
                            ),
                            detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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
                        .get(&key_resource_body_urlencoded_param_order(
                            entry_id, &param_id,
                        ))
                        .and_then(|value| serde_json::from_value(value.clone()).ok()),
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
                            title: localize!(
                                "workbench.activity.failed_to_convert_value_expression",
                                "Failed to convert value expression"
                            ),
                            detail: Some(localize!(NO_TRANSLATE_KEY, err)),
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
                        .get(&key_resource_body_formdata_param_order(entry_id, &param_id))
                        .and_then(|value| serde_json::from_value(value.clone()).ok()),
                });
            }
            BodyInfo::FormData(param_infos)
        }
    };
    Some(inner)
}
