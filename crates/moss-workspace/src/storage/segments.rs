use moss_storage::primitives::segkey::SegKey;

pub static SEGKEY_EXPANDED_ITEMS: SegKey = SegKey::new("expandedItems");
pub static SEGKEY_EXPANDED_ENVIRONMENT_GROUPS: SegKey = SegKey::new("expandedEnvironmentGroups");
pub static SEGKEY_SELECTED_ENVIRONMENTS: SegKey = SegKey::new("selectedEnvironments");

pub static SEGKEY_ENVIRONMENT_GROUP: SegKey = SegKey::new("environmentGroup");
pub static SEGKEY_COLLECTION: SegKey = SegKey::new("collection");
pub static SEGKEY_ENVIRONMENT: SegKey = SegKey::new("environment");
pub static SEGKEY_LAYOUT: SegKey = SegKey::new("layout");

pub static SEGKEY_LAYOUT_SIDEBAR: SegKey = SegKey::new("layout:sidebar");
pub static SEGKEY_LAYOUT_PANEL: SegKey = SegKey::new("layout:panel");
pub static SEGKEY_LAYOUT_EDITOR: SegKey = SegKey::new("layout:editor");
pub static SEGKEY_LAYOUT_ACTIVITYBAR: SegKey = SegKey::new("layout:activitybar");
