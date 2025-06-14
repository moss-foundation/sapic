pub mod common;
pub mod dir;
pub mod item;

pub use common::*;
pub use dir::*;
pub use item::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
