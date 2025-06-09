use serde::{Deserialize, Serialize};
use std::{ops::Deref, path::Path, sync::Arc};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(tag = "type")]
#[ts(export, export_to = "primitives.ts")]
pub enum WorktreeChange {
    Loaded {
        id: Uuid,
        path: Arc<Path>,
    },
    Created {
        id: Uuid,
        path: Arc<Path>,
    },
    Moved {
        id: Uuid,
        from_id: Uuid,
        to_id: Uuid,
        old_path: Arc<Path>,
        new_path: Arc<Path>,
    },
    Deleted {
        id: Uuid,
        path: Arc<Path>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(transparent)]
#[ts(export, export_to = "primitives.ts")]
pub struct WorktreeDiff(Arc<[WorktreeChange]>);

impl Deref for WorktreeDiff {
    type Target = [WorktreeChange];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<WorktreeChange>> for WorktreeDiff {
    fn from(changes: Vec<WorktreeChange>) -> Self {
        Self(Arc::from(changes))
    }
}
