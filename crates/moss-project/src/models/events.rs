use serde::Serialize;
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
