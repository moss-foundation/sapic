use anyhow::{Context, Result};
use moss_common::leased_slotmap::ResourceKey;
use moss_fs::FileSystem;
use moss_workbench::activity_indicator::{ActivityHandle, ActivityIndicator};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{path::PathBuf, sync::Arc};
use tauri::Runtime as TauriRuntime;
use tokio::sync::mpsc;
use tokio::task;

use crate::constants::*;

#[derive(Debug)]
pub struct IndexedRequestEntry {
    pub folder_name: String,
    pub folder_path: PathBuf,
    pub spec_file_path: PathBuf,
}

#[derive(Debug)]
pub struct IndexedRequestGroupEntry {
    pub folder_name: String,
    pub folder_path: PathBuf,
    pub spec_file_path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum IndexedEntry {
    Request(IndexedRequestEntry),
    RequestGroup(IndexedRequestGroupEntry),
}

#[derive(Debug)]
pub struct IndexJob {
    pub collection_key: ResourceKey,
    pub collection_abs_path: PathBuf,
    pub result_tx: mpsc::UnboundedSender<IndexedEntry>,
}

#[derive(Debug, Clone)]
pub struct IndexerHandle {
    tx: mpsc::UnboundedSender<IndexJob>,
}

impl IndexerHandle {
    pub fn new(tx: mpsc::UnboundedSender<IndexJob>) -> Self {
        Self { tx }
    }

    pub fn emit_job(&self, job: IndexJob) -> Result<()> {
        self.tx.send(job).context("Failed to send the index job")?;

        Ok(())
    }
}

pub async fn run<R: tauri::Runtime>(
    activity_indicator: ActivityIndicator<R>,
    fs: Arc<dyn FileSystem>,
    mut rx: mpsc::UnboundedReceiver<IndexJob>,
) {
    while let Some(job) = rx.recv().await {
        let fs_clone = Arc::clone(&fs);
        let activity_indicator_clone = activity_indicator.clone();

        task::spawn(async move {
            if let Err(e) = process_job(fs_clone, activity_indicator_clone, job).await {
                eprintln!("Indexing error: {}", e);
            }
        });
    }
}

async fn process_job<R: TauriRuntime>(
    fs: Arc<dyn FileSystem>,
    activity_indicator: ActivityIndicator<R>,
    job: IndexJob,
) -> Result<()> {
    let total = count_entries(fs.as_ref(), &job.collection_abs_path.join(REQUESTS_DIR)).await?;
    // TODO: count the total number of endpoints, components, etc.
    let progress_counter = Arc::new(AtomicUsize::new(0));

    let activity_id = format!("indexing/{}", job.collection_key);
    let activity_handle =
        activity_indicator.emit_continual(&activity_id, "Indexing".to_string(), None)?;

    let progress_callback = progress_callback(progress_counter.clone(), &activity_handle, total);

    // TODO: traverse the endpoints, components, etc. not just requests
    traverse_requests(
        fs.as_ref(),
        &job.collection_abs_path.join(REQUESTS_DIR),
        &progress_callback,
        job.result_tx,
    )
    .await?;

    Ok(activity_handle.emit_finish()?)
}

async fn count_entries(fs: &dyn FileSystem, root: &PathBuf) -> Result<usize> {
    let mut count = 0;
    let mut stack: Vec<PathBuf> = vec![root.clone()];

    while let Some(current_dir) = stack.pop() {
        let mut dir = fs.read_dir(&current_dir).await.context(format!(
            "Failed to read the directory: {}",
            current_dir.display()
        ))?;

        while let Some(entry) = dir.next_entry().await? {
            count += 1;
            println!("entry: {}, count: {}", &entry.path().display(), count);

            let file_type = entry.file_type().await?;

            if file_type.is_dir() {
                stack.push(entry.path());
            }
        }
    }

    Ok(count)
}

async fn traverse_requests(
    fs: &dyn FileSystem,
    root: &PathBuf,
    progress_callback: &impl Fn(&PathBuf) -> Result<()>,
    result_tx: mpsc::UnboundedSender<IndexedEntry>,
) -> Result<()> {
    let mut stack: Vec<PathBuf> = vec![root.clone()];

    while let Some(current_dir) = &stack.pop() {
        let mut dir = fs.read_dir(&current_dir).await.context(format!(
            "Failed to read the directory: {}",
            current_dir.display()
        ))?;

        while let Some(entry) = dir.next_entry().await? {
            let entry_path = entry.path();

            {
                progress_callback(&entry_path)?;
            }


            // In the requests folder, we have either request entries or request groups
            if is_request_entry_dir(&entry_path) {
                let entry_result = index_request_entry_dir(fs, &entry_path, progress_callback)
                    .await
                    .context(format!(
                        "Failed to index the request folder: {}",
                        entry_path.display()
                    ))?;

                result_tx.send(entry_result).context(format!(
                    "Failed to send the indexed request folder to the result channel: {}",
                    entry_path.display()
                ))?;
            } else if entry_path.is_dir(){
                stack.push(entry_path.clone());
                let spec_file_path = entry_path.join(FOLDER_ENTRY_SPEC_FILE);
                let entry = IndexedRequestGroupEntry {
                    folder_name: entry
                        .file_name()
                        .to_string_lossy()
                        .to_string(),
                    folder_path: entry_path.clone(),
                    spec_file_path: if spec_file_path.exists() {
                        Some(spec_file_path)
                    } else {
                        None
                    }
                };

                result_tx
                    .send(IndexedEntry::RequestGroup(entry))
                    .context(format!(
                        "Failed to send the indexed request folder to the result channel: {}",
                        entry_path.display()
                    ))?;
            }
        }
    }

    Ok(())
}

async fn index_request_entry_dir(
    fs: &dyn FileSystem,
    path: &PathBuf,
    progress_callback: &impl Fn(&PathBuf) -> Result<()>,
) -> Result<IndexedEntry> {
    let mut inner_dir = fs.read_dir(&path).await?;

    let folder_name = path
        .file_name()
        .context("Failed to read the request group folder name")?
        .to_string_lossy()
        .to_string();

    let mut spec_file_abs_path = None;

    while let Some(inner_entry) = inner_dir.next_entry().await? {
        {
            progress_callback(&inner_entry.path())?;
        }

        let entry_path = inner_entry.path();
        let entry_metadata = inner_entry.metadata().await?;

        if spec_file_abs_path.is_none()
            && entry_metadata.is_file()
            && is_entry_spec_file(&entry_path)
        {
            spec_file_abs_path = Some(entry_path);
            continue;
        }
    }

    Ok(IndexedEntry::Request(IndexedRequestEntry {
        folder_name,
        folder_path: path.to_path_buf(),
        spec_file_path: spec_file_abs_path
            .ok_or_else(|| anyhow::anyhow!("No spec file found in the request folder"))?,
    }))
}

pub(crate) fn is_request_entry_dir(entry_path: &PathBuf) -> bool {
    entry_path.is_dir()
        && entry_path
            .extension()
            .map(|ext| ext == FOLDER_ENTRY_DIR_EXT)
            .unwrap_or(false)
}

pub(crate) fn is_folder_entry_spec_file(file_path: &PathBuf) -> bool {
    file_path
        .file_name()
        .map(|name| name.to_string_lossy().to_string() == FOLDER_ENTRY_SPEC_FILE)
        .unwrap_or(false)
}

pub(crate) fn is_entry_spec_file(file_path: &PathBuf) -> bool {
    match file_path.file_name() {
        Some(name) => {
            name.to_string_lossy().to_string() == GET_ENTRY_SPEC_FILE
                || name.to_string_lossy().to_string() == POST_ENTRY_SPEC_FILE
                || name.to_string_lossy().to_string() == PUT_ENTRY_SPEC_FILE
                || name.to_string_lossy().to_string() == DELETE_ENTRY_SPEC_FILE
                || name.to_string_lossy().to_string() == GRAPHQL_ENTRY_SPEC_FILE
                || name.to_string_lossy().to_string() == GRPC_ENTRY_SPEC_FILE
        }
        None => false,
    }
}

fn progress_callback<'a, R: TauriRuntime>(
    progress_counter: Arc<AtomicUsize>,
    activity_handle: &'a ActivityHandle<'_, R>,
    total: usize,
) -> impl Fn(&PathBuf) -> Result<()> + 'a {
    move |path| {
        progress_counter.fetch_add(1, Ordering::SeqCst);
        let current = progress_counter.load(Ordering::SeqCst);

        activity_handle.emit_progress(Some(format!(
            "{}/{} ({})",
            current,
            total,
            path.to_string_lossy().to_string()
        )))?;

        Ok(())
    }
}
