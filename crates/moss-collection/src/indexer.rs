use anyhow::{Context as _, Result};
use moss_common::leased_slotmap::ResourceKey;
use moss_fs::FileSystem;
use moss_workbench::activity_indicator::{ActivityHandle, ActivityIndicator};
use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{path::PathBuf, sync::Arc};
use tauri::Emitter;
use tauri::Runtime as TauriRuntime;
use tokio::sync::mpsc;
use tokio::task;

const INDEXING_ACTIVITY_ID: &str = "indexing";

#[derive(Debug, Clone)]
pub struct IndexedItem {
    pub collection_key: ResourceKey,
    pub path: PathBuf,
}

pub struct IndexJob {
    pub collection_key: ResourceKey,
    pub collection_abs_path: PathBuf,
    pub result_tx: mpsc::UnboundedSender<IndexedItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IndexingEvent {
    pub collection_key: ResourceKey,
    pub progress_percent: u32,
    pub path: PathBuf,
}

pub async fn run<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    activity_indicator: ActivityIndicator<R>,
    fs: Arc<dyn FileSystem>,
    mut rx: mpsc::UnboundedReceiver<IndexJob>,
) {
    while let Some(job) = rx.recv().await {
        let fs_clone = Arc::clone(&fs);
        let app_handle_clone = app_handle.clone();
        let activity_indicator_clone = activity_indicator.clone();

        task::spawn(async move {
            if let Err(e) =
                process_job(fs_clone, app_handle_clone, activity_indicator_clone, job).await
            {
                eprintln!("Indexing error: {}", e);
            }
        });
    }
}

async fn process_job<R: TauriRuntime>(
    fs: Arc<dyn FileSystem>,
    app_handle: tauri::AppHandle<R>,
    activity_indicator: ActivityIndicator<R>,
    job: IndexJob,
) -> Result<()> {
    let total = count_entries(fs.as_ref(), &job.collection_abs_path.join("requests")).await?;
    let progress_counter = Arc::new(AtomicUsize::new(0));

    let activity_handle = activity_indicator.emit_continual(
        INDEXING_ACTIVITY_ID,
        "Indexing".to_string(),
        format!("0/{}", total),
        Some(0),
    )?;

    traverse_requests(
        fs.as_ref(),
        activity_handle,
        &job.collection_abs_path.join("requests"),
        job.collection_key,
        total,
        progress_counter,
    )
    .await?;

    Ok(())
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
            dbg!(&entry.path());

            let file_type = entry.file_type().await?;
            if file_type.is_dir() {
                stack.push(entry.path());
            }
        }
    }

    Ok(count)
}

async fn traverse_requests<R: TauriRuntime>(
    fs: &dyn FileSystem,
    activity_handle: ActivityHandle<'_, R>,
    root: &PathBuf,
    collection_key: ResourceKey,
    total: usize,
    progress_counter: Arc<AtomicUsize>,
) -> Result<()> {
    let mut stack: Vec<PathBuf> = vec![root.clone()];

    while let Some(current_dir) = stack.pop() {
        let mut dir = fs.read_dir(&current_dir).await.context(format!(
            "Failed to read the directory: {}",
            current_dir.display()
        ))?;

        while let Some(entry) = dir.next_entry().await? {
            progress_counter.fetch_add(1, Ordering::SeqCst);
            let current = progress_counter.load(Ordering::SeqCst);
            let progress_percent = (current as f64 / total as f64 * 100.0) as u32;

            activity_handle.emit_progress(
                progress_percent as u8,
                format!(
                    "{}/{} ({})",
                    current,
                    total,
                    entry.path().to_string_lossy().to_string()
                ),
            )?;

            let file_type = entry.file_type().await?;
            if file_type.is_dir() {
                stack.push(entry.path());
            }
        }
    }

    Ok(())
}
