use crate::{CreateOptions, FileSystem, FsError, FsResult, RemoveOptions, RenameOptions};
use async_stream::stream;
use async_zip::{
    Compression, ZipEntryBuilder,
    tokio::{read::fs::ZipFileReader, write::ZipFileWriter},
};
use atomic_fs::Rollback;
use futures::{StreamExt, stream::BoxStream};
use joinerror::{OptionExt, ResultExt, bail};
use nanoid::nanoid;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use sapic_core::{
    context,
    context::{AnyAsyncContext, ContextResultExt},
};
use std::{io, path::Path, sync::Arc, time::Duration};
use tokio::{
    fs::{OpenOptions, ReadDir},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
    time::{self, Instant},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub struct RealFileSystem {
    tmp: Arc<Path>,
}

impl RealFileSystem {
    pub fn new(tmp: &Path) -> Self {
        Self { tmp: tmp.into() }
    }
}

#[async_trait::async_trait]
impl FileSystem for RealFileSystem {
    async fn create_dir_all(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            tokio::fs::create_dir_all(path)
                .await
                .join_err::<()>("failed to create dir recursively")
        })
        .await
        .join_err_bare()
    }

    async fn create_dir(&self, ctx: &dyn AnyAsyncContext, path: &Path) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            tokio::fs::create_dir(path)
                .await
                .join_err::<()>("failed to create dir recursively")
        })
        .await
        .join_err_bare()
    }

    async fn remove_dir(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        options: RemoveOptions,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            (if options.recursive {
                tokio::fs::remove_dir_all(path).await
            } else {
                tokio::fs::remove_dir(path).await
            })
            .or_else(|err| {
                if err.kind() == io::ErrorKind::NotFound && options.ignore_if_not_exists {
                    Ok(())
                } else {
                    bail!("failed to remove dir: {}", err.to_string());
                }
            })
        })
        .await
        .join_err_bare()
    }

    async fn create_file(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        options: CreateOptions,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            let mut open_options = tokio::fs::OpenOptions::new();
            open_options.write(true).create(true);
            if options.overwrite {
                open_options.truncate(true);
            } else if !options.ignore_if_exists {
                open_options.create_new(true);
            }

            open_options
                .open(path)
                .await
                .join_err::<()>("failed to create file")?;

            Ok(())
        })
        .await
        .join_err_bare()
    }

    async fn is_dir_empty(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
    ) -> joinerror::Result<bool> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            if !path.is_dir() {
                bail!("path `{}` is not a directory", path.display());
            }

            let mut entries = tokio::fs::read_dir(path).await?;
            Ok(entries.next_entry().await?.is_none())
        })
        .await
        .join_err_bare()
    }

    async fn create_file_with(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        content: &[u8],
        options: CreateOptions,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
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
        })
        .await
        .join_err_bare()
    }

    async fn remove_file(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        options: RemoveOptions,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            tokio::fs::remove_file(path).await.or_else(|err| {
                if err.kind() == io::ErrorKind::NotFound && options.ignore_if_not_exists {
                    Ok(())
                } else {
                    Err(err)?
                }
            })
        })
        .await
        .join_err_bare()
    }

    async fn open_file(
        &self,
        _ctx: &dyn AnyAsyncContext,
        path: &Path,
    ) -> joinerror::Result<Box<dyn io::Read + Send + Sync>> {
        Ok(Box::new(std::fs::File::open(path)?))
    }

    async fn rename(
        &self,
        ctx: &dyn AnyAsyncContext,
        from: &Path,
        to: &Path,
        options: RenameOptions,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            if !options.overwrite && tokio::fs::metadata(to).await.is_ok() {
                if options.ignore_if_exists {
                    return Ok(());
                } else {
                    bail!("Path {} already exists", to.display())
                }
            }

            Ok(tokio::fs::rename(from, to).await?)
        })
        .await
        .join_err_bare()
    }

    async fn read_dir(&self, ctx: &dyn AnyAsyncContext, path: &Path) -> joinerror::Result<ReadDir> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            Ok(tokio::fs::read_dir(path).await?)
        })
        .await
        .join_err_bare()
    }

    fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> joinerror::Result<(
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
        )
        .map_err(|err| {
            joinerror::Error::new::<()>(format!(
                "failed to watch `{}`: {}",
                path.display(),
                err.to_string()
            ))
        })?;

        if let Err(e) = watcher.watch(path, RecursiveMode::Recursive) {
            bail!("failed to watch path `{}`: {}", path.display(), e);
        }

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

    async fn zip(
        &self,
        ctx: &dyn AnyAsyncContext,
        src_dir: &Path,
        out_file: &Path,
        excluded_entries: &[&str],
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            if !src_dir.is_dir() {
                bail!("directory `{}` does not exist", src_dir.display());
            }

            // If the output archive file is inside the source folder, it will also be bundled, which we don't want
            if out_file.starts_with(src_dir) {
                bail!("cannot export archive file into the source directory");
            }

            let src_dir = src_dir.to_path_buf();
            let out_file = out_file.to_path_buf();

            let mut output_writer =
                ZipFileWriter::with_tokio(tokio::fs::File::create(&out_file).await?);

            // Operations
            let mut dirs = vec![src_dir.clone()];
            while let Some(path) = dirs.pop() {
                let mut read_dir = tokio::fs::read_dir(path).await?;
                while let Some(entry) = read_dir.next_entry().await? {
                    let path = entry.path();
                    let file_name = entry.file_name().to_string_lossy().to_string();

                    if file_name.is_empty() || excluded_entries.contains(&file_name.as_str()) {
                        continue;
                    }

                    let file_type = entry.file_type().await?;

                    if file_type.is_dir() {
                        dirs.push(path);
                        continue;
                    }

                    // Write the entry to the zip file
                    let mut input_file = tokio::fs::File::open(&path).await?;
                    let size = input_file.metadata().await?.len();

                    let mut buffer = Vec::with_capacity(size as usize);
                    input_file.read_to_end(&mut buffer).await?;

                    let entry_path = path
                        .strip_prefix(&src_dir)
                        .expect("children must have source directory as prefix");
                    let entry_str = entry_path
                        .as_os_str()
                        .to_str()
                        .ok_or_join_err::<()>("entry has non-utf-8 characters")?;

                    let builder = ZipEntryBuilder::new(entry_str.into(), Compression::Deflate);
                    output_writer
                        .write_entry_whole(builder, &buffer)
                        .await
                        .map_err(|err| {
                            joinerror::Error::new::<()>(format!(
                                "failed to write entry to archive file: {}",
                                err
                            ))
                        })?;
                }
            }
            output_writer.close().await.map_err(|err| {
                joinerror::Error::new::<()>(format!("failed to close archive file: {}", err))
            })?;
            Ok(())
        })
        .await
        .join_err_bare()
    }

    async fn unzip(
        &self,
        ctx: &dyn AnyAsyncContext,
        src_archive: &Path,
        out_dir: &Path,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            if !out_dir.is_dir() {
                bail!("directory `{}` does not exist", out_dir.display());
            }

            let reader = ZipFileReader::new(src_archive).await.map_err(|err| {
                joinerror::Error::new::<()>(format!(
                    "failed to read archive `{}`: {}",
                    src_archive.display(),
                    err
                ))
            })?;

            for index in 0..reader.file().entries().len() {
                let entry = &reader.file().entries()[index];
                let path =
                    out_dir.join(entry.filename().as_str().map_err(|_| {
                        FsError::Other("archive entry has non-UTF-8 path".to_string())
                    })?);

                let mut entry_reader = reader.reader_without_entry(index).await.map_err(|err| {
                    FsError::Other(format!("failed to read entry in the archive file: {}", err))
                })?;

                let parent = path
                    .parent()
                    .expect("parent must exist since out_dir is valid");
                if !parent.is_dir() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                let writer = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&path)
                    .await?;

                futures::io::copy(&mut entry_reader, &mut writer.compat_write()).await?;
            }

            Ok(())
        })
        .await
        .join_err_bare()
    }

    // Create a folder for a particular rollback session
    async fn start_rollback(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<Rollback> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            let session_tmp = self.tmp.join(nanoid!(10));
            tokio::fs::create_dir(&session_tmp)
                .await
                .join_err::<()>("failed to start a fs rollback session")?;
            Ok(Rollback::new(session_tmp).await)
        })
        .await
        .join_err_bare()
    }

    async fn create_dir_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            atomic_fs::create_dir(rb, path).await
        })
        .await
        .join_err_bare()
    }

    async fn create_dir_all_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            atomic_fs::create_dir_all(rb, path).await
        })
        .await
        .join_err_bare()
    }

    async fn remove_dir_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
        options: RemoveOptions,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            atomic_fs::remove_dir(
                rb,
                path,
                atomic_fs::RemoveOptions {
                    ignore_if_not_exists: options.ignore_if_not_exists,
                },
            )
            .await
        })
        .await
        .join_err_bare()
    }

    async fn create_file_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
        options: CreateOptions,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            atomic_fs::create_file(
                rb,
                path,
                atomic_fs::CreateOptions {
                    overwrite: options.overwrite,
                    ignore_if_exists: options.ignore_if_exists,
                },
            )
            .await
        })
        .await
        .join_err_bare()
    }

    async fn create_file_with_content_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
        content: &[u8],
        options: CreateOptions,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            atomic_fs::create_file_with(
                rb,
                path,
                atomic_fs::CreateOptions {
                    overwrite: options.overwrite,
                    ignore_if_exists: options.ignore_if_exists,
                },
                content,
            )
            .await
        })
        .await
        .join_err_bare()
    }

    async fn remove_file_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
        options: RemoveOptions,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            atomic_fs::remove_file(
                rb,
                path,
                atomic_fs::RemoveOptions {
                    ignore_if_not_exists: options.ignore_if_not_exists,
                },
            )
            .await
        })
        .await
        .join_err_bare()
    }

    async fn rename_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        from: &Path,
        to: &Path,
        options: RenameOptions,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            atomic_fs::rename(
                rb,
                from,
                to,
                atomic_fs::RenameOptions {
                    overwrite: options.overwrite,
                    ignore_if_exists: options.ignore_if_exists,
                },
            )
            .await
        })
        .await
        .join_err_bare()
    }
}
