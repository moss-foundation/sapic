use joinerror::ResultExt;
use moss_fs::FileSystem;
use regorus::Value as RegoValue;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::manifest::ThemeFile;

pub struct ThemeLoader {
    fs: Arc<dyn FileSystem>,
    policy_path: PathBuf,
}

impl ThemeLoader {
    pub fn new(fs: Arc<dyn FileSystem>, policy_path: PathBuf) -> Self {
        Self { fs, policy_path }
    }

    pub async fn load(&self, path: &Path) -> joinerror::Result<ThemeFile> {
        let rdr = self.fs.open_file(path).await?;
        let file: ThemeFile = serde_json::from_reader(rdr)?;

        let mut buf = String::new();
        let mut policy_rdr = self.fs.open_file(&self.policy_path).await?;
        policy_rdr.read_to_string(&mut buf)?;

        self.validate(&file, buf)?;

        Ok(file)
    }

    fn validate(&self, theme: &ThemeFile, policy_content: String) -> joinerror::Result<()> {
        let mut engine = regorus::Engine::new();

        engine
            .add_policy("theme.rego".to_string(), policy_content)
            .join_err::<()>("failed to add theme rego policy")?;

        engine.set_input(RegoValue::from(serde_json::to_value(theme)?));

        let result = engine
            .eval_rule("data.theme.errors".to_string())
            .join_err::<()>("failed to evaluate theme rego rule")?;

        let errors = result
            .as_set()?
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(joinerror::Error::new::<()>(format!(
                "invalid theme:\n{}",
                errors.join("\n")
            )))
        }
    }
}
