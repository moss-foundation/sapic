use moss_applib::EventMarker;
use moss_user::models::primitives::ProfileId;
use rustc_hash::{FxHashMap, FxHashSet};
use serde_json::Value as JsonValue;

use crate::models::primitives::WorkspaceId;

#[derive(Debug, Clone)]
pub struct OnDidChangeProfile {
    pub id: ProfileId,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct OnDidChangeWorkspace {
    pub id: WorkspaceId,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct OnDidChangeConfiguration {
    pub affected_keys: FxHashSet<String>,
    pub changes: FxHashMap<String, JsonValue>,
}

impl EventMarker for OnDidChangeConfiguration {}
impl EventMarker for OnDidChangeProfile {}
impl EventMarker for OnDidChangeWorkspace {}
