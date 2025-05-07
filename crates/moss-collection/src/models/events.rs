use serde::Serialize;
use ts_rs::TS;

use super::operations::EntryInfo;

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "events.ts")]
pub struct StreamEntriesByPrefixesEvent(pub Vec<EntryInfo>);
