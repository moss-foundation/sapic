use anyhow::anyhow;
use derive_more::{Deref, DerefMut};
use joinerror::OptionExt;
use moss_applib::{AppRuntime, ServiceMarker};
use moss_common::{continue_if_err, continue_if_none};
use moss_db::primitives::AnyValue;
use moss_fs::{CreateOptions, FileSystem, RemoveOptions, desanitize_path, utils::SanitizedPath};
use moss_hcl::HclResultExt;
use moss_storage::primitives::segkey::SegKeyBuf;
use moss_text::sanitized::{desanitize, sanitize};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    fs,
    sync::{RwLock, mpsc},
};

use crate::{
    constants,
    constants::{COLLECTION_ROOT_PATH, DIR_CONFIG_FILENAME},
    models::{
        primitives::{EntryClass, EntryId, EntryKind, EntryProtocol},
        types::configuration::docschema::{RawDirConfiguration, RawItemConfiguration},
    },
    storage::segments,
};

use crate::{
    errors::{ErrorAlreadyExists, ErrorInvalidInput, ErrorInvalidKind, ErrorIo, ErrorNotFound},
    services::storage_service::StorageService,
};

#[derive(Debug)]
struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    scan_queue: mpsc::UnboundedSender<ScanJob>,
}

pub struct EntryMetadata {
    pub order: isize,
    pub expanded: bool,
}

pub enum EntryConfiguration {
    Item(RawItemConfiguration),
    Dir(RawDirConfiguration),
}

impl EntryConfiguration {
    pub fn as_item(&self) -> Option<&RawItemConfiguration> {
        match self {
            EntryConfiguration::Item(conf) => Some(conf),
            EntryConfiguration::Dir(_) => None,
        }
    }

    pub fn as_dir(&self) -> Option<&RawDirConfiguration> {
        match self {
            EntryConfiguration::Item(_) => None,
            EntryConfiguration::Dir(conf) => Some(conf),
        }
    }

    pub fn classification(&self) -> EntryClass {
        match self {
            EntryConfiguration::Item(conf) => match conf {
                RawItemConfiguration::Request(_) => EntryClass::Request,
                RawItemConfiguration::Endpoint(_) => EntryClass::Endpoint,
                RawItemConfiguration::Component(_) => EntryClass::Component,
                RawItemConfiguration::Schema(_) => EntryClass::Schema,
            },
            EntryConfiguration::Dir(conf) => conf.classification(),
        }
    }

    pub fn protocol(&self) -> Option<EntryProtocol> {
        match self {
            EntryConfiguration::Item(conf) => match conf {
                RawItemConfiguration::Request(conf) => conf.url.protocol(),
                RawItemConfiguration::Endpoint(conf) => conf.url.protocol(),
                RawItemConfiguration::Component(_) => None,
                RawItemConfiguration::Schema(_) => None,
            },
            EntryConfiguration::Dir(_) => None,
        }
    }

    pub fn kind(&self) -> EntryKind {
        match self {
            EntryConfiguration::Item(_) => EntryKind::Item,
            EntryConfiguration::Dir(_) => EntryKind::Dir,
        }
    }
}

pub struct ModifyParams {
    pub name: Option<String>,
    pub protocol: Option<EntryProtocol>,
    pub expanded: Option<bool>,
    pub order: Option<isize>,
    pub path: Option<PathBuf>,
}

#[derive(Deref, DerefMut)]
pub struct EntryItemMut<'a> {
    path: &'a mut Arc<Path>,

    #[deref]
    #[deref_mut]
    configuration: &'a mut RawItemConfiguration,
}

#[derive(Deref, DerefMut)]
pub struct EntryDirMut<'a> {
    path: &'a mut Arc<Path>,

    #[deref]
    #[deref_mut]
    configuration: &'a mut RawDirConfiguration,
}

#[derive(Deref, DerefMut)]
pub(crate) struct Entry {
    id: EntryId,
    path: Arc<Path>,

    #[deref]
    #[deref_mut]
    configuration: EntryConfiguration,
}

impl Entry {
    pub fn as_item_mut(&mut self) -> Option<EntryItemMut<'_>> {
        match &mut self.configuration {
            EntryConfiguration::Item(configuration) => Some(EntryItemMut {
                path: &mut self.path,
                configuration,
            }),
            _ => None,
        }
    }

    pub fn as_dir_mut(&mut self) -> Option<EntryDirMut<'_>> {
        match &mut self.configuration {
            EntryConfiguration::Dir(configuration) => Some(EntryDirMut {
                path: &mut self.path,
                configuration,
            }),
            _ => None,
        }
    }
}

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

pub struct WorktreeService<R: AppRuntime> {
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    storage: Arc<StorageService<R>>,
    state: Arc<RwLock<WorktreeState>>,
}

impl<R: AppRuntime> ServiceMarker for WorktreeService<R> {}

// FIXME: Should we attach error markers?

impl<R: AppRuntime> WorktreeService<R> {
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

        let abs_path = self.absolutize(&entry.path)?;
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

                let dir_name = job
                    .path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| job.path.to_string_lossy().to_string());

                match process_dir_entry(&job.path, &fs, &job.abs_path).await {
                    Ok(Some(entry)) => {
                        let expanded = expanded_entries.contains(&entry.id);
                        let desc = EntryDescription {
                            id: entry.id.clone(),
                            name: desanitize(&dir_name),
                            path: entry.path.clone(),
                            class: entry.classification(),
                            kind: entry.kind(),
                            protocol: entry.configuration.protocol(),
                            expanded,
                            order: all_entry_keys
                                .get(&segments::segkey_entry_order(&entry.id))
                                .and_then(|o| o.deserialize().ok()),
                        };

                        let _ = sender.send(desc);
                        if expanded {
                            state
                                .write()
                                .await
                                .expanded_entries
                                .insert(entry.id.clone());
                        }
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
                        continue_if_err!(process_dir_entry(&child_path, &fs, &child_abs_path).await)
                    } else {
                        continue_if_err!(
                            process_file_entry(&child_name, &child_path, &fs, &child_abs_path)
                                .await
                        )
                    };

                    let entry = continue_if_none!(maybe_entry, || {
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
                        let desc = EntryDescription {
                            id: entry.id.clone(),
                            name: desanitize(&child_name),
                            path: entry.path.clone(),
                            class: entry.classification(),
                            kind: entry.kind(),
                            protocol: entry.configuration.protocol(),
                            expanded: expanded_entries.contains(&entry.id),
                            order: all_entry_keys
                                .get(&segments::segkey_entry_order(&entry.id))
                                .and_then(|o| o.deserialize().ok()),
                        };

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
        id: &EntryId,
        name: &str,
        path: &Path,
        configuration: RawItemConfiguration,
        metadata: EntryMetadata,
    ) -> joinerror::Result<()> {
        debug_assert!(path.is_relative());

        if !is_parent_a_dir_entry(self.abs_path.as_ref(), path) {
            return Err(joinerror::Error::new::<ErrorInvalidInput>(format!(
                "Cannot create entry inside Item entry {}",
                path.to_string_lossy().to_string()
            )));
        }

        let sanitized_path: SanitizedPath = moss_fs::utils::sanitize_path(path, None)?
            .join(sanitize(name))
            .into();

        let content = hcl::to_string(&configuration)
            .join_err::<()>("failed to serialize configuration into hcl string")?;
        self.create_entry(&sanitized_path, false, &content.as_bytes())
            .await?;

        let mut state_lock = self.state.write().await;
        state_lock.entries.insert(
            id.to_owned(),
            Entry {
                id: id.to_owned(),
                path: sanitized_path.to_path_buf().into(),
                configuration: EntryConfiguration::Item(configuration),
            },
        );

        {
            let mut txn = self.storage.begin_write(ctx).await?;

            self.storage
                .put_entry_order_txn(ctx, &mut txn, id, metadata.order)
                .await?;

            if metadata.expanded {
                state_lock.expanded_entries.insert(id.to_owned());

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
        id: &EntryId,
        name: &str,
        path: &Path,
        configuration: RawDirConfiguration,
        metadata: EntryMetadata,
    ) -> joinerror::Result<()> {
        debug_assert!(path.is_relative());

        if !is_parent_a_dir_entry(self.abs_path.as_ref(), path) {
            return Err(joinerror::Error::new::<ErrorInvalidInput>(format!(
                "Cannot create entry inside Item entry {}",
                path.to_string_lossy().to_string()
            )));
        }

        let sanitized_path: SanitizedPath = moss_fs::utils::sanitize_path(path, None)?
            .join(sanitize(name))
            .into();

        let content = hcl::to_string(&configuration)
            .join_err::<()>("failed to serialize configuration into hcl string")?;
        self.create_entry(&sanitized_path, true, &content.as_bytes())
            .await?;

        let mut state_lock = self.state.write().await;
        state_lock.entries.insert(
            id.to_owned(),
            Entry {
                id: id.to_owned(),
                path: sanitized_path.to_path_buf().into(),
                configuration: EntryConfiguration::Dir(configuration),
            },
        );

        {
            let mut txn = self.storage.begin_write(ctx).await?;
            self.storage
                .put_entry_order_txn(ctx, &mut txn, id, metadata.order)
                .await?;

            if metadata.expanded {
                state_lock.expanded_entries.insert(id.to_owned());

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
    ) -> joinerror::Result<(PathBuf, RawDirConfiguration)> {
        let mut state_lock = self.state.write().await;
        let entry = state_lock
            .entries
            .get_mut(&id)
            .ok_or_join_err_with::<ErrorNotFound>(|| format!("entry {} not found", id))?
            .as_dir_mut()
            .ok_or_join_err_with::<ErrorInvalidKind>(|| {
                format!("entry {} is not a directory", id)
            })?;

        let mut path = entry.path.clone().to_path_buf();

        if let Some(new_parent) = params.path {
            let classification_folder = entry.configuration.classification_folder();
            if !new_parent.starts_with(classification_folder) {
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

            let old_path = entry.path.clone();
            let new_path = update_path_parent(entry.path.as_ref(), &new_parent)?;
            path = new_path.clone();

            self.rename_entry(&old_path, &new_path).await?;
            *entry.path = new_path.into();
        }

        if let Some(name) = params.name {
            let old_path = entry.path.clone();
            let new_path = rename_path(entry.path.as_ref(), &name);
            path = new_path.clone();

            self.rename_entry(&old_path, &new_path).await?;
            *entry.path = new_path.into();
        }

        let configuration = entry.configuration.clone();
        let is_db_update_needed = params.order.is_some() || params.expanded.is_some();
        if !is_db_update_needed {
            return Ok((path, configuration));
        }

        let mut txn = self.storage.begin_write(ctx).await?;

        if let Some(order) = params.order {
            self.storage
                .put_entry_order_txn(ctx, &mut txn, id, order)
                .await?;
        }

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

        Ok((path, configuration))
    }

    pub async fn update_item_entry(
        &self,
        ctx: &R::AsyncContext,
        id: &EntryId,
        params: ModifyParams,
    ) -> joinerror::Result<(PathBuf, RawItemConfiguration)> {
        let mut state_lock = self.state.write().await;
        let entry = state_lock
            .entries
            .get_mut(&id)
            .ok_or_join_err_with::<ErrorNotFound>(|| format!("entry {} not found", id))?
            .as_item_mut()
            .ok_or_join_err_with::<ErrorInvalidKind>(|| format!("entry {} is not a item", id))?;

        let mut path = entry.path.clone().to_path_buf();

        if let Some(new_parent) = params.path {
            let classification_folder = entry.configuration.classification_folder();
            if !new_parent.starts_with(classification_folder) {
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

            let old_path = entry.path.clone();
            let new_path = update_path_parent(entry.path.as_ref(), &new_parent)?;
            path = new_path.clone();

            self.rename_entry(&old_path, &new_path).await?;
            *entry.path = new_path.into();
        }

        if let Some(name) = params.name {
            let old_path = entry.path.clone();
            let new_path = rename_path(entry.path.as_ref(), &name);
            path = new_path.clone();

            self.rename_entry(&old_path, &new_path).await?;
            *entry.path = new_path.into();
        }

        if let Some(protocol) = params.protocol {
            match entry.configuration {
                RawItemConfiguration::Request(request) => {
                    request.change_protocol(protocol);

                    Ok(())
                }
                RawItemConfiguration::Endpoint(endpoint) => {
                    endpoint.change_protocol(protocol);

                    Ok(())
                }
                RawItemConfiguration::Component(_) => {
                    Err(joinerror::Error::new::<ErrorInvalidInput>(
                        "cannot set protocol for component item",
                    ))
                }
                RawItemConfiguration::Schema(_) => Err(joinerror::Error::new::<ErrorInvalidInput>(
                    "cannot set protocol for schema item",
                )),
            }?;
        }

        let configuration = entry.configuration.clone();
        let is_db_update_needed = params.order.is_some() || params.expanded.is_some();
        if !is_db_update_needed {
            return Ok((path, configuration));
        }

        let mut txn = self.storage.begin_write(ctx).await?;

        if let Some(order) = params.order {
            self.storage
                .put_entry_order_txn(ctx, &mut txn, id, order)
                .await?;
        }

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

        Ok((path, configuration))
    }
}

impl<R: AppRuntime> WorktreeService<R> {
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

impl<R: AppRuntime> WorktreeService<R> {
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

    async fn rename_entry(&self, from: &Path, to: &Path) -> joinerror::Result<()> {
        // On Windows and macOS, file/directory names are case-preserving but insensitive
        // If the from and to path differs only with different casing of the filename
        // The rename should still succeed

        let abs_from = self.absolutize(&from)?;
        let abs_to = self.absolutize(&to)?;

        if !abs_from.exists() {
            return Err(joinerror::Error::new::<ErrorNotFound>(format!(
                "entry not found: {}",
                abs_from.display()
            )));
        }

        let old_name_lower = from
            .file_name()
            .map(|name| name.to_string_lossy().to_lowercase())
            .ok_or_join_err::<ErrorIo>("invalid file name")?;
        let new_name_lower = to
            .file_name()
            .map(|name| name.to_string_lossy().to_lowercase())
            .ok_or_join_err::<ErrorIo>("invalid file name")?;
        let recasing_only =
            old_name_lower == new_name_lower && abs_from.parent() == abs_to.parent();

        if abs_to.exists() && !recasing_only {
            return Err(joinerror::Error::new::<ErrorAlreadyExists>(format!(
                "entry already exists: {}",
                to.display()
            )));
        }

        self.fs
            .rename(
                &abs_from,
                &abs_to,
                moss_fs::RenameOptions {
                    overwrite: true,
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

// We don't allow creating subentries inside an item
fn is_parent_a_dir_entry(abs_path: &Path, parent_path: &Path) -> bool {
    if parent_path == Path::new(COLLECTION_ROOT_PATH) {
        // Ignore the root level since it's not an entry
        return true;
    }
    abs_path
        .join(parent_path)
        .join(constants::DIR_CONFIG_FILENAME)
        .exists()
}

async fn process_dir_entry(
    path: &Arc<Path>,
    fs: &Arc<dyn FileSystem>,
    abs_path: &Path,
) -> joinerror::Result<Option<Entry>> {
    let dir_config_path = abs_path.join(constants::DIR_CONFIG_FILENAME);
    let item_config_path = abs_path.join(constants::ITEM_CONFIG_FILENAME);

    if dir_config_path.exists() {
        let config = parse_configuration::<RawDirConfiguration>(&fs, &dir_config_path).await?;

        return Ok(Some(Entry {
            id: config.id().clone(),
            path: desanitize_path(path, None)?.into(),
            configuration: EntryConfiguration::Dir(config),
        }));
    }

    if item_config_path.exists() {
        let config = parse_configuration::<RawItemConfiguration>(&fs, &item_config_path).await?;

        return Ok(Some(Entry {
            id: config.id().clone(),
            path: desanitize_path(path, None)?.into(),
            configuration: EntryConfiguration::Item(config),
        }));
    }

    Ok(None)
}

async fn process_file_entry(
    _name: &str,
    _path: &Arc<Path>,
    _fs: &Arc<dyn FileSystem>,
    _abs_path: &Path,
) -> joinerror::Result<Option<Entry>> {
    // TODO: implement
    Ok(None)
}

async fn parse_configuration<T>(fs: &Arc<dyn FileSystem>, path: &Path) -> joinerror::Result<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let mut reader = fs.open_file(path).await?;
    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .map_err(|e| joinerror::Error::new::<ErrorIo>(e.to_string()))?;

    Ok(hcl::from_str(&buf).map_err(anyhow::Error::from)?)
}
