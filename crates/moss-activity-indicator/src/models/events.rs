use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Clone, TS)]
#[ts(export, export_to = "types.ts")]
#[serde(rename_all = "camelCase")]
pub enum ActivityEvent<'a> {
    /// This event is used when the activity is a one-time event
    /// and we don't want to track its progress.
    #[serde(rename_all = "camelCase")]
    Oneshot {
        id: usize,
        activity_id: &'a str,
        title: String,
        #[ts(optional)]
        detail: Option<String>,
    },
    /// This event is used when the activity is a long-running event
    /// and we want to track its progress, like indexing, scanning, etc.
    #[serde(rename_all = "camelCase")]
    Start {
        id: usize,
        activity_id: &'a str,
        title: String,
        #[ts(optional)]
        detail: Option<String>,
    },
    /// This event is used to update the progress of a long-running activity,
    /// like updating the progress of an indexer, scanner, etc.
    #[serde(rename_all = "camelCase")]
    Progress {
        id: usize,
        activity_id: &'a str,
        #[ts(optional)]
        detail: Option<String>,
    },
    /// This event is used to notify the frontend that the long-running activity
    /// is finished and the activity indicator should be hidden.
    #[serde(rename_all = "camelCase")]
    Finish { id: usize, activity_id: &'a str },
}
