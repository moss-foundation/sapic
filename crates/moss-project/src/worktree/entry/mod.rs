pub mod edit;
pub mod model;

use derive_more::{Deref, DerefMut};
use sapic_base::resource::types::primitives::{ResourceId, *};
use std::{path::Path, sync::Arc};
use tokio::sync::watch;

use crate::worktree::entry::{edit::EntryEditing, model::BodyKind};

#[derive(Deref, DerefMut)]
pub(crate) struct Entry {
    pub id: ResourceId,
    pub path_rx: watch::Receiver<Arc<Path>>,
    #[allow(unused)]
    pub class: ResourceClass,
    pub protocol: Option<ResourceProtocol>,
    pub url: Option<String>,
    pub metadata: EntryMetadata,
    #[deref]
    #[deref_mut]
    pub edit: EntryEditing,
}

pub(crate) struct EntryMetadata {
    pub body_kind: Option<BodyKind>,
}

#[derive(Debug)]
pub struct ScannedEntry {
    pub id: ResourceId,
    pub name: String,
    pub path: Arc<Path>,
    pub class: ResourceClass,
    pub kind: ResourceKind,
    pub protocol: Option<ResourceProtocol>,
}
