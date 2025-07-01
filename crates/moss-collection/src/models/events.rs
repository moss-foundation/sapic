use serde::Serialize;
use ts_rs::TS;

use crate::models::types::{AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription};

use super::types::EntryInfo;

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "events.ts")]
pub struct StreamEntriesEvent(pub EntryInfo);

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "events.ts")]
pub enum BatchUpdateEntryEvent {
    Item(AfterUpdateItemEntryDescription),
    Dir(AfterUpdateDirEntryDescription),
}
