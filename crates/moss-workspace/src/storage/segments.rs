use moss_storage::primitives::segkey::SegKey;

pub(crate) static ROOT_COLLECTION_SEGKEY: SegKey = SegKey::new("collection");
pub(crate) static ROOT_ENVIRONMENT_SEGKEY: SegKey = SegKey::new("environment");
pub(crate) static ROOT_PART_SEGKEY: SegKey = SegKey::new("part");

pub(crate) static SIDEBAR_PART_SEGKEY: SegKey = SegKey::new("part:sidebar");
pub(crate) static PANEL_PART_SEGKEY: SegKey = SegKey::new("part:panel");
pub(crate) static EDITOR_PART_SEGKEY: SegKey = SegKey::new("part:editor");
