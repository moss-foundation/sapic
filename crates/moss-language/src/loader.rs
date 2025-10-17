use moss_fs::FileSystem;
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};

pub struct LanguageLoader {
    fs: Arc<dyn FileSystem>,
}

impl LanguageLoader {
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self { fs }
    }

    pub async fn load_namespace(&self, path: &Path, ns: &str) -> joinerror::Result<JsonValue> {
        let abs_path = path.join(format!("{}.json", ns));
        let rdr = self.fs.open_file(&abs_path).await?;
        let parsed: JsonValue = serde_json::from_reader(rdr)?;

        Ok(parsed)
    }
}
