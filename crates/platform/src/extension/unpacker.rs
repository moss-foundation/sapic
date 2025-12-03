use flate2::bufread::GzDecoder;
use joinerror::OptionExt;
use moss_fs::{FileSystem, RemoveOptions};
use std::{fs::File, io::BufReader, path::Path, sync::Arc};
use tar::Archive;

pub struct ExtensionUnpacker;

impl ExtensionUnpacker {
    pub async fn unpack(
        archive_path: &Path,
        destination_path: &Path,
        fs: Arc<dyn FileSystem>,
    ) -> joinerror::Result<()> {
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

            fs.remove_file(
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
