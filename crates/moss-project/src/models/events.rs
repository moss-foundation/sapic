use sapic_base::resource::types::primitives::{ResourceId, *};
use sapic_ipc::contracts::main::resource::FrontendResourcePath;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::types::{AfterUpdateDirResourceDescription, AfterUpdateItemResourceDescription};

/// @category Event
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "events.ts")]
pub enum BatchUpdateResourceEvent {
    Item(AfterUpdateItemResourceDescription),
    Dir(AfterUpdateDirResourceDescription),
}
