use joinerror::Error;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
use zip::write::SimpleFileOptions;

pub(crate) async fn zip_dir(
    src_dir: &Path,
    out_file: &Path,
    excluded_dirs: Vec<String>,
    excluded_files: Vec<String>,
) -> joinerror::Result<()> {
    // If the output archive file is inside the source folder, it will also be bundled, which we don't want
    if out_file.starts_with(src_dir) {
        return Err(Error::new::<()>(
            "cannot export archive file into the source folder",
        ));
    }

    // Wrap synchronous zip operations so that it doesn't block async runtime
    let src_dir = src_dir.to_path_buf();
    let out_file = out_file.to_path_buf();

    tokio::task::spawn_blocking(move || {
        let file = File::create(out_file)?;
        let mut zip = zip::ZipWriter::new(file);

        let options = SimpleFileOptions::default();

        // Recursively add every file into the archive, excluding certain directories and files
        let mut dirs = vec![src_dir.clone()];
        let mut buffer = Vec::new();
        while let Some(path) = dirs.pop() {
            let mut read_dir = std::fs::read_dir(path)?;
            while let Some(entry) = read_dir.next() {
                let entry = entry?;
                let path = entry.path();
                let file_type = entry.file_type()?;

                // Skip special paths
                let file_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if file_name.is_empty() {
                    continue;
                }

                let relative_path = path
                    .strip_prefix(&src_dir)
                    .expect("Children entries must have this prefix");

                if file_type.is_dir() && !excluded_dirs.contains(&file_name) {
                    zip.add_directory_from_path(&relative_path, options)
                        .map_err(|err| {
                            Error::new::<()>(format!(
                                "failed to add directory to the archive: {}",
                                err
                            ))
                        })?;
                    dirs.push(path.clone());
                }

                if file_type.is_file() && !excluded_files.contains(&file_name) {
                    zip.start_file_from_path(relative_path, options)
                        .map_err(|err| {
                            Error::new::<()>(format!(
                                "failed to start creating file in the archive: {}",
                                err
                            ))
                        })?;

                    let mut f = File::open(path)?;
                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&buffer)?;
                    buffer.clear();
                }
            }
        }

        Ok::<(), Error>(())
    })
    .await??;

    Ok(())
}

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
