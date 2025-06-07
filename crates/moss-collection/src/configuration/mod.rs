pub mod dir;
pub mod item;

pub use dir::*;
pub use item::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SpecificationMetadata {
    pub id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigurationModel {
    Item(ItemConfigurationModel),
    Dir(DirConfigurationModel),
}

impl ConfigurationModel {
    pub fn id(&self) -> Uuid {
        match self {
            ConfigurationModel::Item(item) => item.metadata.id,
            ConfigurationModel::Dir(dir) => dir.metadata.id,
        }
    }
}
