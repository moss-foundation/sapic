use moss_storage::primitives::segkey::SegKey;

pub(crate) static COLLECTION_SEGKEY: SegKey = SegKey::new("collection");
pub(crate) static ENVIRONMENT_SEGKEY: SegKey = SegKey::new("environment");
pub(crate) static PART_SEGKEY: SegKey = SegKey::new("part");

pub(crate) static PART_SIDEBAR_SEGKEY: SegKey = SegKey::new("part:sidebar");
pub(crate) static PART_PANEL_SEGKEY: SegKey = SegKey::new("part:panel");
pub(crate) static PART_EDITOR_SEGKEY: SegKey = SegKey::new("part:editor");
