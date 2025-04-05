use anyhow::{Context as _, Result};
use async_trait::async_trait;
use moss_common::leased_slotmap::ResourceKey;
use moss_fs::FileSystem;
use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{path::PathBuf, sync::Arc};
use tauri::Emitter;
use tauri::Runtime as TauriRuntime;
use tokio::sync::mpsc;
use tokio::task;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::Stream;

pub struct IndexedItem {
    pub collection_key: ResourceKey,
    pub path: PathBuf,
}

pub struct IndexJob {
    pub collection_key: ResourceKey,
    pub collection_abs_path: PathBuf,
    pub result_tx: mpsc::Sender<IndexedItem>,
}

// pub struct CollectionIndexer {
//     fs: Arc<dyn FileSystem>,
//     app_handle: tauri::AppHandle,
//     // rx: mpsc::UnboundedReceiver<(mpsc::UnboundedSender<IndexedItem>, IndexJob)>,
//     tx: mpsc::UnboundedSender<(mpsc::UnboundedSender<IndexedItem>, IndexJob)>,
// }

#[derive(Debug, Clone, Serialize)]
pub struct IndexingEvent {
    pub collection_key: ResourceKey,
    pub progress_percent: u32,
    pub path: PathBuf,
}

pub async fn run<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    fs: Arc<dyn FileSystem>,
    mut rx: mpsc::UnboundedReceiver<(mpsc::UnboundedSender<IndexedItem>, IndexJob)>,
) {
    while let Some(job) = rx.recv().await {
        let fs_clone = Arc::clone(&fs);
        let app_handle_clone = app_handle.clone();

        task::spawn(async move {
            if let Err(e) = process_job(fs_clone, app_handle_clone, job).await {
                eprintln!("Indexing error: {}", e);
            }
        });
    }
}

async fn process_job<R: TauriRuntime>(
    fs: Arc<dyn FileSystem>,
    app_handle: tauri::AppHandle<R>,
    job: (mpsc::UnboundedSender<IndexedItem>, IndexJob),
) -> Result<()> {
    let total = count_entries(fs.as_ref(), &job.1.collection_abs_path).await?;
    let progress_counter = Arc::new(AtomicUsize::new(0));

    traverse_requests(
        fs.as_ref(),
        app_handle,
        &job.1.collection_abs_path.join("requests"),
        job.1.collection_key,
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
    app_handle: tauri::AppHandle<R>,
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

            let _ = app_handle.emit(
                "test-indexing",
                IndexingEvent {
                    collection_key,
                    progress_percent,
                    path: entry.path(),
                },
            );

            let file_type = entry.file_type().await?;
            if file_type.is_dir() {
                stack.push(entry.path());
            }
        }
    }

    Ok(())
}

// impl CollectionIndexer {
//     pub fn new(
//         app_handle: tauri::AppHandle,
//         fs: Arc<dyn FileSystem>,
//         tx: mpsc::UnboundedSender<(mpsc::UnboundedSender<IndexedItem>, IndexJob)>,
//         // mut rx: mpsc::Receiver<IndexJob>,
//     ) -> Arc<Self> {
//         // let (tx, mut rx) = mpsc::unbounded_channel();
//         let indexer = Arc::new(Self { fs, app_handle, tx });

//         // while let Some(job) = rx.recv().await {
//         //     let indexer_clone = indexer.clone();

//         //     task::spawn(async move {
//         //         if let Err(e) = indexer_clone.process_job(job).await {
//         //             eprintln!("Indexing error: {}", e);
//         //         }
//         //     });
//         // }

//         indexer
//     }

//     pub async fn run(
//         self: Arc<Self>,
//         mut rx: mpsc::UnboundedReceiver<(mpsc::UnboundedSender<IndexedItem>, IndexJob)>,
//     ) {
//         while let Some(job) = rx.recv().await {
//             let indexer_clone = self.clone();

//             task::spawn(async move {
//                 if let Err(e) = indexer_clone.process_job(job).await {
//                     eprintln!("Indexing error: {}", e);
//                 }
//             });
//         }
//     }

//     async fn process_job(
//         self: Arc<Self>,
//         job: (mpsc::UnboundedSender<IndexedItem>, IndexJob),
//     ) -> Result<()> {
//         let total = self.count_entries(&job.1.collection_abs_path).await?;
//         let progress_counter = Arc::new(AtomicUsize::new(0));

//         self.traverse_requests(
//             &job.1.collection_abs_path.join("requests"),
//             job.1.collection_key,
//             total,
//             progress_counter,
//         )
//         .await?;

//         Ok(())
//     }

//     async fn count_entries(self: &Arc<Self>, root: &PathBuf) -> Result<usize> {
//         let mut count = 0;
//         let mut stack: Vec<PathBuf> = vec![root.clone()];

//         while let Some(current_dir) = stack.pop() {
//             let mut dir = self.fs.read_dir(&current_dir).await.context(format!(
//                 "Failed to read the directory: {}",
//                 current_dir.display()
//             ))?;

//             while let Some(entry) = dir.next_entry().await? {
//                 count += 1;

//                 let file_type = entry.file_type().await?;
//                 if file_type.is_dir() {
//                     stack.push(entry.path());
//                 }
//             }
//         }

//         Ok(count)
//     }

//     async fn traverse_requests(
//         self: Arc<Self>,
//         root: &PathBuf,
//         collection_key: ResourceKey,
//         total: usize,
//         progress_counter: Arc<AtomicUsize>,
//     ) -> Result<()> {
//         let mut stack: Vec<PathBuf> = vec![root.clone()];

//         while let Some(current_dir) = stack.pop() {
//             let mut dir = self.fs.read_dir(&current_dir).await.context(format!(
//                 "Failed to read the directory: {}",
//                 current_dir.display()
//             ))?;

//             while let Some(entry) = dir.next_entry().await? {
//                 progress_counter.fetch_add(1, Ordering::SeqCst);
//                 let current = progress_counter.load(Ordering::SeqCst);
//                 let progress_percent = (current as f64 / total as f64 * 100.0) as u32;

//                 let _ = self.app_handle.emit(
//                     "test-indexing",
//                     IndexingEvent {
//                         collection_key,
//                         progress_percent,
//                         path: entry.path(),
//                     },
//                 );

//                 let file_type = entry.file_type().await?;
//                 if file_type.is_dir() {
//                     stack.push(entry.path());
//                 }
//             }
//         }

//         Ok(())
//     }
// }
