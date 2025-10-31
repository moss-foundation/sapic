use moss_applib::EventMarker;

use crate::models::primitives::StorageScope;

#[derive(Debug, Clone)]
pub struct OnDidChangeValueEvent {
    pub key: String,
    pub scope: StorageScope,
    pub removed: bool,
}

impl EventMarker for OnDidChangeValueEvent {}
