pub mod edit;
pub mod model;

use derive_more::{Deref, DerefMut};
use std::{path::Path, sync::Arc};
use tokio::sync::watch;

use crate::{
    models::primitives::{EntryClass, EntryId, EntryKind, EntryProtocol},
    worktree::entry::edit::EntryEditing,
};

#[derive(Deref, DerefMut)]
pub(crate) struct Entry {
    pub id: EntryId,
    pub path_rx: watch::Receiver<Arc<Path>>,
    #[allow(unused)]
    pub class: EntryClass,
    pub protocol: Option<EntryProtocol>,

    #[deref]
    #[deref_mut]
    pub edit: EntryEditing,
}

#[derive(Debug)]
pub struct EntryDescription {
    pub id: EntryId,
    pub name: String,
    pub path: Arc<Path>,
    pub class: EntryClass,
    pub kind: EntryKind,
    pub protocol: Option<EntryProtocol>,
    pub order: Option<isize>,
    pub expanded: bool,
}
