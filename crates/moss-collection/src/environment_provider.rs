use std::{path::Path, sync::Arc};

use joinerror::ResultExt;
use moss_applib::AppRuntime;
use moss_environment::Environment;
use moss_fs::FileSystem;
use tokio::sync::mpsc::UnboundedSender;

use crate::dirs;

pub(crate) struct EnvironmentProvider {
    fs: Arc<dyn FileSystem>,
    abs_path: Arc<Path>,
}

impl EnvironmentProvider {
    pub fn new(fs: Arc<dyn FileSystem>, abs_path: Arc<Path>) -> Self {
        Self { fs, abs_path }
    }

    pub async fn scan<R: AppRuntime>(
        &self,
        aggregation_tx: UnboundedSender<Environment<R>>,
    ) -> joinerror::Result<()> {
        let abs_path = self.abs_path.join(dirs::ENVIRONMENTS_DIR);
        let mut read_dir = self.fs.read_dir(&abs_path).await.map_err(|err| {
            joinerror::Error::new::<()>(format!(
                "failed to read directory {} : {}",
                abs_path.display(),
                err
            ))
        })?;

        // while let Some(entry) = read_dir.next_entry().await? {
        //     if entry.file_type().await?.is_dir() {
        //         continue;
        //     }
        // }

        Ok(())
    }
}
