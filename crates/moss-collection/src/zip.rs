use joinerror::Error;
use std::path::Path;

pub(crate) async fn unzip_dir(src_archive: &Path, out_dir: &Path) -> joinerror::Result<()> {
    // Wrap synchronous zip operations so that it doesn't block async runtime
    let src_archive = src_archive.to_path_buf();
    let out_dir = out_dir.to_path_buf();

    tokio::task::spawn_blocking(move || {
        // Wrap synchronous zip operations so that it doesn't block async runtime
        let archive_file = std::fs::File::open(&src_archive)?;
        let mut archive = zip::ZipArchive::new(archive_file).map_err(|err| {
            Error::new::<()>(format!(
                "failed to open archive file `{}`: {}",
                src_archive.display(),
                err
            ))
        })?;

        archive.extract(&out_dir).map_err(|err| {
            Error::new::<()>(format!(
                "failed to extract archive file `{}`: {}",
                out_dir.display(),
                err
            ))
        })?;

        Ok::<(), Error>(())
    })
    .await??;

    Ok(())
}
