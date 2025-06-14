pub mod dir;
pub mod item;

pub use dir::*;
pub use item::*;

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ConfigurationMetadata {
    pub id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigurationModel {
    Item(CompositeItemConfigurationModel),
    Dir(CompositeDirConfigurationModel),
}

impl ConfigurationModel {
    pub fn id(&self) -> Uuid {
        match self {
            ConfigurationModel::Item(item) => item.metadata.id,
            ConfigurationModel::Dir(dir) => dir.metadata.id,
        }
    }
}
