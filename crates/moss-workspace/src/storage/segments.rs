use moss_storage::primitives::segkey::SegKey;

pub static SEGKEY_EXPANDED_ITEMS: SegKey = SegKey::new("expandedItems");
pub static COLLECTION_SEGKEY: SegKey = SegKey::new("collection");
pub static ENVIRONMENT_SEGKEY: SegKey = SegKey::new("environment");
pub static LAYOUT_SEGKEY: SegKey = SegKey::new("layout");

pub static LAYOUT_SIDEBAR_SEGKEY: SegKey = SegKey::new("layout:sidebar");
pub static LAYOUT_PANEL_SEGKEY: SegKey = SegKey::new("layout:panel");
pub static LAYOUT_EDITOR_SEGKEY: SegKey = SegKey::new("layout:editor");
pub static LAYOUT_ACTIVITYBAR_SEGKEY: SegKey = SegKey::new("layout:activitybar");
