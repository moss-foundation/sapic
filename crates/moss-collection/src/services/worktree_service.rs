use anyhow::anyhow;
use derive_more::{Deref, DerefMut};
use moss_applib::ServiceMarker;
use moss_common::{api::OperationError, continue_if_err, continue_if_none};
use moss_db::primitives::AnyValue;
use moss_fs::{
    CreateOptions, FileSystem, FsError, RemoveOptions, desanitize_path, utils::SanitizedPath,
};
use moss_storage::primitives::segkey::SegKeyBuf;
use moss_text::sanitized::{desanitize, sanitize};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
};
use thiserror::Error;
use tokio::{
    fs,
    sync::{RwLock, mpsc},
};

use crate::{
    constants,
    constants::DIR_CONFIG_FILENAME,
    models::{
        primitives::{EntryClass, EntryId, EntryKind, EntryProtocol},
        types::configuration::docschema::{RawDirConfiguration, RawItemConfiguration},
    },
    services::storage_service::StorageService,
    storage::segments,
};

#[derive(Error, Debug)]
pub enum WorktreeError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("invalid kind: {0}")]
    InvalidKind(String),

    #[error("worktree entry already exists: {0}")]
    AlreadyExists(String),

    #[error("worktree entry is not found: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl From<moss_fs::FsError> for WorktreeError {
    fn from(error: FsError) -> Self {
        WorktreeError::Io(error.to_string())
    }
}

impl From<hcl::Error> for WorktreeError {
    fn from(error: hcl::Error) -> Self {
        WorktreeError::Io(error.to_string())
    }
}

impl From<moss_db::common::DatabaseError> for WorktreeError {
    fn from(error: moss_db::common::DatabaseError) -> Self {
        WorktreeError::Internal(error.to_string())
    }
}

impl From<serde_json::Error> for WorktreeError {
    fn from(error: serde_json::Error) -> Self {
        WorktreeError::Internal(error.to_string())
    }
}

impl From<WorktreeError> for OperationError {
    fn from(error: WorktreeError) -> Self {
        match error {
            WorktreeError::InvalidInput(err) => OperationError::InvalidInput(err),
            WorktreeError::InvalidKind(err) => OperationError::InvalidInput(err),
            WorktreeError::AlreadyExists(err) => OperationError::AlreadyExists(err),
            WorktreeError::NotFound(err) => OperationError::NotFound(err),
            WorktreeError::Unknown(err) => OperationError::Unknown(err),
            WorktreeError::Io(err) => OperationError::Internal(err.to_string()),
            WorktreeError::Internal(err) => OperationError::Internal(err.to_string()),
        }
    }
}

pub type WorktreeResult<T> = Result<T, WorktreeError>;

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

pub struct WorktreeService {
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    state: Arc<RwLock<WorktreeState>>,
    storage: Arc<StorageService>, // TODO: should be a trait
}

impl ServiceMarker for WorktreeService {}

impl WorktreeService {
    pub fn new(abs_path: Arc<Path>, fs: Arc<dyn FileSystem>, storage: Arc<StorageService>) -> Self {
        Self {
            abs_path,
            fs,
            storage,
            state: Default::default(),
        }
    }

    pub fn absolutize(&self, path: &Path) -> WorktreeResult<PathBuf> {
        debug_assert!(path.is_relative());

        if path
            .components()
            .any(|c| c == std::path::Component::ParentDir)
        {
            return Err(WorktreeError::InvalidInput(format!(
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

    pub async fn remove_entry(&self, id: &EntryId) -> WorktreeResult<()> {
        let mut state_lock = self.state.write().await;
        let entry = state_lock
            .entries
            .remove(&id)
            .ok_or(WorktreeError::NotFound(id.to_string()))?;

        let abs_path = self.absolutize(&entry.path)?;
        if !abs_path.exists() {
            return Err(WorktreeError::NotFound(format!(
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
        self.storage.put_expanded_entries(
            state_lock
                .expanded_entries
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
        )?;

        Ok(())
    }

    pub async fn scan(
        &self,
        path: &Path,
        expanded_entries: Arc<HashSet<EntryId>>,
        all_entry_keys: Arc<HashMap<SegKeyBuf, AnyValue>>,
        sender: mpsc::UnboundedSender<EntryDescription>,
    ) -> WorktreeResult<()> {
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
                                .map(|o| o.to_owned().into()),
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
                                .map(|o| o.to_owned().into()),
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
        id: &EntryId,
        name: &str,
        path: impl AsRef<Path>,
        configuration: RawItemConfiguration,
        metadata: EntryMetadata,
    ) -> WorktreeResult<()> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        if !is_parent_entry_dir(self.abs_path.as_ref(), path) {
            return Err(WorktreeError::InvalidInput(format!(
                "Cannot create entry inside Item entry {}",
                path.to_string_lossy().to_string()
            )));
        }

        let sanitized_path: SanitizedPath = moss_fs::utils::sanitize_path(path, None)?
            .join(sanitize(name))
            .into();

        let content = hcl::to_string(&configuration)?;
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
            let mut txn = self.storage.begin_write()?;

            self.storage
                .put_entry_order_txn(&mut txn, id, metadata.order)?;

            if metadata.expanded {
                state_lock.expanded_entries.insert(id.to_owned());

                self.storage.put_expanded_entries_txn(
                    &mut txn,
                    state_lock
                        .expanded_entries
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                )?;
            }

            txn.commit()?;
        }

        Ok(())
    }

    pub async fn create_dir_entry(
        &self,
        id: &EntryId,
        name: &str,
        path: impl AsRef<Path>,
        configuration: RawDirConfiguration,
        metadata: EntryMetadata,
    ) -> WorktreeResult<()> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        if !is_parent_entry_dir(self.abs_path.as_ref(), path) {
            return Err(WorktreeError::InvalidInput(format!(
                "Cannot create entry inside Item entry {}",
                path.to_string_lossy().to_string()
            )));
        }

        let sanitized_path: SanitizedPath = moss_fs::utils::sanitize_path(path, None)?
            .join(sanitize(name))
            .into();

        let content = hcl::to_string(&configuration)?;
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
            let mut txn = self.storage.begin_write()?;
            self.storage
                .put_entry_order_txn(&mut txn, id, metadata.order)?;

            if metadata.expanded {
                state_lock.expanded_entries.insert(id.to_owned());

                self.storage.put_expanded_entries_txn(
                    &mut txn,
                    state_lock
                        .expanded_entries
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                )?;
            }

            txn.commit()?;
        }

        Ok(())
    }

    pub async fn update_dir_entry(
        &self,
        id: &EntryId,
        params: ModifyParams,
    ) -> WorktreeResult<(PathBuf, RawDirConfiguration)> {
        let mut state_lock = self.state.write().await;
        let entry = state_lock
            .entries
            .get_mut(&id)
            .ok_or(WorktreeError::NotFound(id.to_string()))?
            .as_dir_mut()
            .ok_or(WorktreeError::InvalidKind(
                "expected to be a dir".to_string(),
            ))?;

        let mut path = entry.path.clone().to_path_buf();

        if let Some(new_parent) = params.path {
            let classification_folder = entry.configuration.classification_folder();
            if !new_parent.starts_with(classification_folder) {
                return Err(WorktreeError::InvalidInput(
                    "Cannot move entry to a different classification folder".to_string(),
                ));
            }

            // We can only move entries into a directory entry
            // Check if the destination path has dir config file
            let dest_entry_config = self.abs_path.join(&new_parent).join(DIR_CONFIG_FILENAME);
            if !dest_entry_config.exists() {
                return Err(WorktreeError::InvalidInput(
                    "Cannot move entries into a non-directory entry".to_string(),
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

        let mut txn = self.storage.begin_write()?;

        if let Some(order) = params.order {
            self.storage.put_entry_order_txn(&mut txn, &id, order)?;
        }

        if let Some(expanded) = params.expanded {
            if expanded {
                state_lock.expanded_entries.insert(id.to_owned());
            } else {
                state_lock.expanded_entries.remove(id);
            }

            self.storage.put_expanded_entries_txn(
                &mut txn,
                state_lock
                    .expanded_entries
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>(),
            )?;
        }

        txn.commit()?;

        Ok((path, configuration))
    }

    pub async fn update_item_entry(
        &self,
        id: &EntryId,
        params: ModifyParams,
    ) -> WorktreeResult<(PathBuf, RawItemConfiguration)> {
        let mut state_lock = self.state.write().await;
        let entry = state_lock
            .entries
            .get_mut(&id)
            .ok_or(WorktreeError::NotFound(id.to_string()))?
            .as_item_mut()
            .ok_or(WorktreeError::InvalidKind(
                "expected to be an item".to_string(),
            ))?;

        let mut path = entry.path.clone().to_path_buf();

        if let Some(new_parent) = params.path {
            let classification_folder = entry.configuration.classification_folder();
            if !new_parent.starts_with(classification_folder) {
                return Err(WorktreeError::InvalidInput(
                    "Cannot move entry to a different classification folder".to_string(),
                ));
            }

            // We can only move entries into a directory entry
            // Check if the destination path has dir config file
            let dest_entry_config = self.abs_path.join(&new_parent).join(DIR_CONFIG_FILENAME);
            if !dest_entry_config.exists() {
                return Err(WorktreeError::InvalidInput(
                    "Cannot move entries into a non-directory entry".to_string(),
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
                RawItemConfiguration::Component(_) => Err(WorktreeError::InvalidInput(
                    "Cannot set protocol for component item".to_string(),
                )),
                RawItemConfiguration::Schema(_) => Err(WorktreeError::InvalidInput(
                    "Cannot set protocol for schema item".to_string(),
                )),
            }?;
        }

        let configuration = entry.configuration.clone();
        let is_db_update_needed = params.order.is_some() || params.expanded.is_some();
        if !is_db_update_needed {
            return Ok((path, configuration));
        }

        let mut txn = self.storage.begin_write()?;

        if let Some(order) = params.order {
            self.storage.put_entry_order_txn(&mut txn, &id, order)?;
        }

        if let Some(expanded) = params.expanded {
            if expanded {
                state_lock.expanded_entries.insert(id.to_owned());
            } else {
                state_lock.expanded_entries.remove(id);
            }

            self.storage.put_expanded_entries_txn(
                &mut txn,
                state_lock
                    .expanded_entries
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>(),
            )?;
        }

        txn.commit()?;

        Ok((path, configuration))
    }

    async fn create_entry(
        &self,
        path: &SanitizedPath,
        is_dir: bool,
        content: &[u8],
    ) -> WorktreeResult<()> {
        let abs_path = self.absolutize(&path)?;
        if abs_path.exists() {
            return Err(WorktreeError::AlreadyExists(format!(
                "Entry already exists: {}",
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

    async fn rename_entry(&self, from: &Path, to: &Path) -> WorktreeResult<()> {
        let abs_from = self.absolutize(&from)?;
        let abs_to = self.absolutize(&to)?;

        if !abs_from.exists() {
            return Err(WorktreeError::NotFound(from.display().to_string()));
        }

        if abs_to.exists() {
            return Err(WorktreeError::AlreadyExists(to.display().to_string()));
        }

        self.fs
            .rename(
                &abs_from,
                &abs_to,
                moss_fs::RenameOptions {
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

// We don't allow creating subentries inside an item
fn is_parent_entry_dir(abs_path: &Path, parent_path: &Path) -> bool {
    if parent_path == Path::new("") {
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
) -> WorktreeResult<Option<Entry>> {
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
) -> WorktreeResult<Option<Entry>> {
    // TODO: implement
    Ok(None)
}

async fn parse_configuration<T>(fs: &Arc<dyn FileSystem>, path: &Path) -> WorktreeResult<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let mut reader = fs.open_file(path).await?;
    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .map_err(|e| WorktreeError::Io(e.to_string()))?;

    Ok(hcl::from_str(&buf).map_err(anyhow::Error::from)?)
}
