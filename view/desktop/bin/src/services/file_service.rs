use moss_fs::FileSystem;
use std::sync::Arc;

pub struct FileService {
    fs: Arc<dyn FileSystem>,
}

impl FileService {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self { fs }
    }
}
