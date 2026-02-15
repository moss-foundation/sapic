use async_trait::async_trait;
use moss_common::{continue_if_err, continue_if_none};
use moss_fs::{CreateOptions, FileSystem, RemoveOptions, utils::SanitizedPath};
use moss_text::sanitized::desanitize;
use sapic_base::resource::{constants::*, errors::*, types::*};
use sapic_core::context::AnyAsyncContext;
use sapic_system::resource::{ResourceBackend, ScannedEntry};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{fs, sync::mpsc};

#[derive(Debug)]
struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    scan_queue: mpsc::UnboundedSender<ScanJob>,
}

pub struct ResourceFsBackend {
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
}

impl ResourceFsBackend {
    pub fn new(abs_path: PathBuf, fs: Arc<dyn FileSystem>) -> Arc<Self> {
        Self { abs_path, fs }.into()
    }

    fn absolutize(&self, path: &Path) -> joinerror::Result<PathBuf> {
        debug_assert!(path.is_relative());

        if path
            .components()
            .any(|c| c == std::path::Component::ParentDir)
        {
            return Err(joinerror::Error::new::<ErrorPathInvalid>(format!(
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
}

#[async_trait]
impl ResourceBackend for ResourceFsBackend {
    async fn create_entry(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: SanitizedPath,
        content: &[u8],
        is_dir: bool,
    ) -> joinerror::Result<()> {
        let abs_path = self.absolutize(&path.to_path_buf())?;
        if abs_path.exists() {
            return Err(joinerror::Error::new::<ErrorPathAlreadyExists>(format!(
                "entry already exists: {}",
                abs_path.display()
            )));
        }

        self.fs.create_dir(ctx, &abs_path).await?;
        let file_path = if is_dir {
            abs_path.join(DIR_CONFIG_FILENAME)
        } else {
            abs_path.join(ITEM_CONFIG_FILENAME)
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

    async fn remove_entry(&self, ctx: &dyn AnyAsyncContext, path: &Path) -> joinerror::Result<()> {
        let abs_path = self.absolutize(path)?;
        if !abs_path.exists() {
            return Err(joinerror::Error::new::<ErrorPathNotFound>(format!(
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

        Ok(())
    }

    async fn scan(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        sender: mpsc::UnboundedSender<ScannedEntry>,
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

            let ctx_clone = ctx.clone_arc();
            let handle = tokio::spawn(async move {
                let mut new_jobs = Vec::new();
                let ctx_clone = ctx_clone.clone();
                if !job.path.as_os_str().is_empty() {
                    match process_entry(ctx_clone.clone(), job.path.clone(), &fs, &job.abs_path)
                        .await
                    {
                        Ok(Some(desc)) => {
                            if let Err(e) = sender.send(desc) {
                                tracing::debug!(
                                    "Failed to send EntryDescription to tokio mpsc channel: {}",
                                    e
                                );
                            }
                        }
                        Ok(None) => {
                            tracing::info!(
                                "Encountered an empty entry dir: {}",
                                job.abs_path.display()
                            );
                            return;
                        }
                        Err(err) => {
                            tracing::error!(
                                "Error processing dir {}: {}",
                                job.abs_path.display(),
                                err
                            );
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

                    let entry = continue_if_none!(maybe_entry, || {
                        tracing::warn!(
                            "non-entry encountered during scan: {}",
                            child_abs_path.display()
                        );
                    });

                    if child_file_type.is_dir() {
                        new_jobs.push(ScanJob {
                            abs_path: Arc::clone(&child_abs_path),
                            path: child_path,
                            scan_queue: job.scan_queue.clone(),
                        });
                    } else {
                        if let Err(e) = sender.send(entry) {
                            tracing::debug!(
                                "failed to send EntryDescription to tokio mpsc channel: {}",
                                e
                            );
                        }
                    }
                }

                for new_job in new_jobs {
                    if let Err(e) = job.scan_queue.send(new_job) {
                        tracing::debug!("failed to send ScanJob to tokio mpsc channel: {}", e);
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Err(err) = handle.await {
                tracing::error!("error joining job: {}", err);
            }
        }

        Ok(())
    }
}

async fn process_file(
    _name: &str,
    _path: &Arc<Path>,
    _fs: &Arc<dyn FileSystem>,
    _abs_path: &Path,
) -> joinerror::Result<Option<ScannedEntry>> {
    // TODO: implement
    Ok(None)
}

async fn process_entry(
    ctx: Arc<dyn AnyAsyncContext>,
    path: Arc<Path>,
    fs: &Arc<dyn FileSystem>,
    abs_path: &Path,
) -> joinerror::Result<Option<ScannedEntry>> {
    let dir_config_path = abs_path.join(DIR_CONFIG_FILENAME);
    let item_config_path = abs_path.join(ITEM_CONFIG_FILENAME);

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());

    if fs.is_dir_empty(ctx.as_ref(), &abs_path).await? {
        tracing::info!("Deleting empty entry folder: {}", abs_path.display());
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
        // let mut rdr = fs.open_file(ctx.as_ref(), &dir_config_path).await?;
        // let model: EntryModel =
        //     hcl::from_reader(&mut rdr).join_err::<()>("failed to parse dir configuration")?;

        // let id = model.id().clone();
        let desc = ScannedEntry {
            // id: id.clone(),

            // FIXME: should be done in the service layer
            name: desanitize(&name),
            path: path.clone(),
            // class: model.class(),
            kind: ResourceKind::Dir,
            // protocol: None,
        };
        // let (path_tx, path_rx) = watch::channel(desanitize_path(&path, None)?.into());

        return Ok(Some(desc));
    } else if item_config_path.exists() {
        // let mut rdr = fs.open_file(ctx.as_ref(), &item_config_path).await?;
        // let model: EntryModel =
        //     hcl::from_reader(&mut rdr).join_err::<()>("failed to parse item configuration")?;

        // let id = model.id().clone();
        let desc = ScannedEntry {
            // id: id.clone(),
            name: desanitize(&name),
            path: path.clone(),
            // class: model.class(),
            kind: ResourceKind::Item,
            // protocol: model.protocol(),
        };

        // let (path_tx, path_rx) = watch::channel(desanitize_path(&path, None)?.into());

        return Ok(Some(desc));
    }

    Ok(None)
}
