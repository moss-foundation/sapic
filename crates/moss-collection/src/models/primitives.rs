use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
};
use ts_rs::TS;
use uuid::Uuid;

use super::types::PathChangeKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(transparent)]
#[ts(export, export_to = "types.ts")]
pub struct EntryId(usize);

impl EntryId {
    pub fn new(counter: &AtomicUsize) -> Self {
        Self(counter.fetch_add(1, SeqCst))
    }

    pub fn to_usize(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for EntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(tag = "type")]
#[ts(export, export_to = "types.ts")]
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

pub type ChangesDiffSet = Arc<[(Arc<Path>, Uuid, PathChangeKind)]>;
