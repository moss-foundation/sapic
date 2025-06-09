use serde::Serialize;
use ts_rs::TS;
use uuid::Uuid;

use super::types::EntryInfo;

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "events.ts")]
pub struct StreamWorktreeEntriesEvent(pub Vec<EntryInfo>);

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "events.ts")]
pub struct ExpandEntryEvent {
    pub parent_id: Uuid,
    pub entry: EntryInfo,
}
