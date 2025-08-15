use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub enum GitProviderType {
    GitHub,
    // TODO: Support self-hosted gitlab
    GitLab,
}
