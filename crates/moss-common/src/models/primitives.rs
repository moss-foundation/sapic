use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use ts_rs::TS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(transparent)]
#[ts(export, export_to = "primitives.ts")]
pub struct Identifier(usize);

impl Identifier {
    pub fn new(counter: &AtomicUsize) -> Self {
        Self(counter.fetch_add(1, SeqCst))
    }

    pub fn to_usize(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
