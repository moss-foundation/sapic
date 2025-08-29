use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_dir: PathBuf,
    pub github_client_id: String,
    pub gitlab_client_id: String,
}
