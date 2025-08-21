use async_stream::stream;
use futures::{StreamExt, stream::BoxStream};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{io, path::Path, time::Duration};
use tokio::{
    fs::ReadDir,
    io::AsyncWriteExt,
    sync::mpsc,
    time::{self, Instant},
};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{CreateOptions, FileSystem, FsError, FsResult, RemoveOptions, RenameOptions};

pub struct RealFileSystem;

impl RealFileSystem {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FileSystem for RealFileSystem {
    async fn create_dir_all(&self, path: &Path) -> FsResult<()> {
        Ok(tokio::fs::create_dir_all(path).await?)
    }

    async fn create_dir(&self, path: &Path) -> FsResult<()> {
        Ok(tokio::fs::create_dir(path).await?)
    }

    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> FsResult<()> {
        (if options.recursive {
            tokio::fs::remove_dir_all(path).await
        } else {
            tokio::fs::remove_dir(path).await
        })
        .or_else(|err| {
            if err.kind() == io::ErrorKind::NotFound && options.ignore_if_not_exists {
                Ok(())
            } else {
                Err(err)?
            }
        })
    }

    async fn create_file(&self, path: &Path, options: CreateOptions) -> FsResult<()> {
        let mut open_options = tokio::fs::OpenOptions::new();
        open_options.write(true).create(true);
        if options.overwrite {
            open_options.truncate(true);
        } else if !options.ignore_if_exists {
            open_options.create_new(true);
        }

        open_options.open(path).await?;

        Ok(())
    }

    async fn create_file_with(
        &self,
        path: &Path,
        content: &[u8],
        options: CreateOptions,
    ) -> FsResult<()> {
        let mut open_options = tokio::fs::OpenOptions::new();
        open_options.write(true).create(true);
        if options.overwrite {
            open_options.truncate(true);
        } else if !options.ignore_if_exists {
            open_options.create_new(true);
        }

        let mut file = open_options.open(path).await?;
        file.write_all(content).await?;
        file.flush().await?;
        Ok(())
    }

    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> FsResult<()> {
        tokio::fs::remove_file(path).await.or_else(|err| {
            if err.kind() == io::ErrorKind::NotFound && options.ignore_if_not_exists {
                Ok(())
            } else {
                Err(err)?
            }
        })
    }

    async fn open_file(&self, path: &Path) -> FsResult<Box<dyn io::Read + Send + Sync>> {
        Ok(Box::new(std::fs::File::open(path)?))
    }

    async fn rename(&self, from: &Path, to: &Path, options: RenameOptions) -> FsResult<()> {
        if !options.overwrite && tokio::fs::metadata(to).await.is_ok() {
            if options.ignore_if_exists {
                return Ok(());
            } else {
                return Err(FsError::AlreadyExists(format!(
                    "Path {} already exists",
                    to.to_string_lossy().to_string()
                )));
            }
        }

        Ok(tokio::fs::rename(from, to).await?)
    }

    async fn read_dir(&self, path: &Path) -> FsResult<ReadDir> {
        Ok(tokio::fs::read_dir(path).await?)
    }

    fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> FsResult<(
        BoxStream<'static, Vec<notify::Event>>,
        notify::RecommendedWatcher,
    )> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
            move |result| {
                if let Ok(event) = result {
                    let _ = tx.send(event);
                }
            },
            Config::default(),
        )?;

        watcher.watch(path, RecursiveMode::Recursive)?;

        let mut stream_rx = UnboundedReceiverStream::new(rx);
        let stream = stream! {
            let mut buffer = Vec::new();

            while let Some(first) = stream_rx.next().await {
                buffer.push(first);

                // timer that resets on every new event
                let timer = time::sleep(latency);
                tokio::pin!(timer);

                loop {
                    tokio::select! {
                        maybe_evt = stream_rx.next() => match maybe_evt {
                            Some(evt) => {
                                buffer.push(evt);
                                timer.as_mut().reset(Instant::now() + latency);
                            }
                            None => break, // upstream closed
                        },
                        () = &mut timer => break, // silence reached
                    }
                }

                yield std::mem::take(&mut buffer);
            }
        };

        Ok((stream.boxed(), watcher))
    }
}
