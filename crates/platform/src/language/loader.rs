use async_trait::async_trait;
use moss_fs::FileSystem;
use sapic_system::language::LanguagePackLoader as LanguageLoaderPort;
use std::sync::Arc;

pub struct LanguagePackLoader {
    fs: Arc<dyn FileSystem>,
}

impl LanguagePackLoader {
    pub fn new(fs: Arc<dyn FileSystem>) -> Arc<Self> {
        Self { fs }.into()
    }
}

#[async_trait]
impl LanguageLoaderPort for LanguagePackLoader {
    async fn load_namespace(
        &self,
        path: &std::path::Path,
        namespace: &str,
    ) -> joinerror::Result<serde_json::Value> {
        let abs_path = path.join(format!("{}.json", namespace));
        let rdr = self.fs.open_file(&abs_path).await?;
        let parsed: serde_json::Value = serde_json::from_reader(rdr)?;

        Ok(parsed)
    }
}
