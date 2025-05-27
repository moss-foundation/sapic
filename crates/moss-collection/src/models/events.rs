use serde::Serialize;
use ts_rs::TS;

use super::types::EntryInfo;

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "events.ts")]
pub struct StreamWorktreeEntriesEvent(pub Vec<EntryInfo>);
