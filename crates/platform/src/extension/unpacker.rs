use async_trait::async_trait;
use flate2::bufread::GzDecoder;
use joinerror::OptionExt;
use moss_fs::{FileSystem, RemoveOptions};
use std::{fs::File, io::BufReader, path::Path, sync::Arc};
use tar::Archive;

#[async_trait]
pub trait ExtensionUnpacker: Send + Sync {
    async fn unpack(&self, archive_path: &Path, destination_path: &Path) -> joinerror::Result<()>;
}

pub struct ExtensionUnpackerImpl {
    fs: Arc<dyn FileSystem>,
}

impl ExtensionUnpackerImpl {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self { fs }
    }
}

#[async_trait]
impl ExtensionUnpacker for ExtensionUnpackerImpl {
    async fn unpack(&self, archive_path: &Path, destination_path: &Path) -> joinerror::Result<()> {
        if destination_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "An extension already exists at {}",
                destination_path.display()
            )));
        }

        let file_name = archive_path
            .file_name()
            .ok_or_join_err::<()>("archive file has invalid file name")?
            .to_string_lossy()
            .to_string();
        if file_name.ends_with(".tar.gz") {
            let file = BufReader::new(File::open(&archive_path)?);
            // Decompress the gzipped data
            let gz_decoder = GzDecoder::new(file);

            // Create a tar archive from the decompressed data
            let mut archive = Archive::new(gz_decoder);
            archive.unpack(destination_path)?;
            drop(archive);

            self.fs
                .remove_file(
                    &archive_path,
                    RemoveOptions {
                        recursive: false,
                        ignore_if_not_exists: false,
                    },
                )
                .await?;
        } else {
            // We only use .tar.gz for extension archive file for now
            unimplemented!()
        }
        Ok(())
    }
}
