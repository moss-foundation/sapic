use serde::Serialize;
use ts_rs::TS;

use crate::models::types::{AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription};

use super::types::EntryInfo;

/// @category Event
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "events.ts")]
pub struct StreamEntriesEvent(pub EntryInfo);

/// @category Event
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "events.ts")]
pub enum BatchUpdateEntryEvent {
    Item(AfterUpdateItemEntryDescription),
    Dir(AfterUpdateDirEntryDescription),
}
