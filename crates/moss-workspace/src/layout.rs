use anyhow::Result;
use moss_db::{Transaction, primitives::AnyValue};
use moss_storage::{
    WorkspaceStorage, primitives::segkey::SegKeyBuf, storage::operations::TransactionalListByPrefix,
};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, sync::Arc};

use crate::{
    constants::{
        TREE_VIEW_GROUP_COLLECTIONS, TREE_VIEW_GROUP_ENVIRONMENTS, TREE_VIEW_GROUP_MOCK_SERVERS,
    },
    models::{
        primitives::{ActivitybarPosition, SidebarPosition},
        types::{
            ActivitybarItemStateInfo, ActivitybarPartStateInfo, EditorGridState, EditorPanelState,
            EditorPartStateInfo, PanelPartStateInfo, SidebarPartStateInfo,
        },
    },
    storage::segments::{
        PART_ACTIVITYBAR_SEGKEY, PART_EDITOR_SEGKEY, PART_PANEL_SEGKEY, PART_SIDEBAR_SEGKEY,
    },
};

// // ------------------------------------
// // Activitybar
// // ------------------------------------

#[derive(Debug)]
pub struct ActivitybarPreferencesItem {
    pub id: String,
    pub order: Option<usize>,
    pub visible: Option<bool>,
}

#[derive(Debug)]
pub struct ActivitybarPartPreferences {
    pub position: Option<ActivitybarPosition>,
    pub items: Option<Vec<ActivitybarPreferencesItem>>,
}

#[derive(Debug, Clone)]
pub struct ActivitybarItem<'a> {
    pub id: &'a str,
    pub order: usize,
    pub visible: bool,
}

#[derive(Debug)]
pub struct ActivitybarPartDefaults<'a> {
    pub position: ActivitybarPosition,
    pub items: &'a [ActivitybarItem<'a>],
}

const ACTIVITYBAR_DEFAULTS: ActivitybarPartDefaults = ActivitybarPartDefaults {
    position: ActivitybarPosition::Default,
    items: &[
        ActivitybarItem {
            id: TREE_VIEW_GROUP_COLLECTIONS,
            order: 0,
            visible: true,
        },
        ActivitybarItem {
            id: TREE_VIEW_GROUP_ENVIRONMENTS,
            order: 1,
            visible: true,
        },
        ActivitybarItem {
            id: TREE_VIEW_GROUP_MOCK_SERVERS,
            order: 2,
            visible: true,
        },
    ],
};

// #[derive(Debug)]
// pub struct ActivitybarPart {
//     pub defaults: ActivitybarPartDefaults,
//     pub preferences: ActivitybarPartPreferences,
// }

// // ------------------------------------
// // Sidebar
// // ------------------------------------

#[derive(Debug)]
pub struct SidebarPartDefaults {
    position: SidebarPosition,
    size: usize,
    is_visible: bool,
}

#[derive(Debug)]
pub struct SidebarPartPreferences {
    position: Option<SidebarPosition>,
    visible: Option<bool>,
}

const SIDEBAR_DEFAULTS: SidebarPartDefaults = SidebarPartDefaults {
    position: SidebarPosition::Left,
    size: 200,
    is_visible: true,
};

// #[derive(Debug)]
// pub struct SidebarPart {
//     pub defaults: SidebarPartDefaults,
//     pub preferences: SidebarPartPreferences,
//     // pub cache: Option<SidebarPartCache>,
// }

// // ------------------------------------
// // Panel
// // ------------------------------------

#[derive(Debug)]
pub struct PanelPartDefaults {
    size: usize,
    is_visible: bool,
}

#[derive(Debug)]
pub struct PanelPartPreferences {
    visible: Option<bool>,
}

const PANEL_DEFAULTS: PanelPartDefaults = PanelPartDefaults {
    size: 200,
    is_visible: true,
};

// #[derive(Debug)]
// pub struct PanelPart {
//     pub defaults: PanelPartDefaults,
//     pub preferences: PanelPartPreferences,
//     // pub cache: Option<PanelPartCache>,
// }

// // ------------------------------------
// // Editor
// // ------------------------------------

#[derive(Debug)]
pub struct EditorPartDefaults {}

#[derive(Debug)]
pub struct EditorPartPreferences {}

const EDITOR_DEFAULTS: EditorPartDefaults = EditorPartDefaults {};

// #[derive(Debug)]
// pub struct EditorPart {
//     pub defaults: EditorPartDefaults,
//     pub preferences: EditorPartPreferences,
// }

// // ------------------------------------

// pub struct LayoutService {
//     storage: Arc<dyn WorkspaceStorage>, // TODO: replace with just a store when we have a way to begin a transaction from the store
//     activitybar: ActivitybarPart,
//     sidebar: SidebarPart,
//     panel: PanelPart,
//     // editor: EditorPart,
// }

// impl LayoutService {
//     pub fn new(storage: Arc<dyn WorkspaceStorage>) -> Result<Self> {
//         let item_store = storage.item_store();
//         let mut txn = storage.begin_read()?;

//         fn to_option<E, S>(
//             result: Result<AnyValue, DatabaseError>,
//             _: std::marker::PhantomData<E>,
//             convert_fn: impl FnOnce(E) -> S,
//         ) -> Result<Option<S>>
//         where
//             E: DeserializeOwned,
//         {
//             match result {
//                 Ok(value) => {
//                     let entity: E = value.deserialize()?;
//                     Ok(Some(convert_fn(entity)))
//                 }
//                 Err(DatabaseError::NotFound { .. }) => Ok(None),
//                 Err(err) => Err(err.into()),
//             }
//         }

//         // ------------------------------------
//         // Activitybar
//         // ------------------------------------

//         // let activitybar_result = TransactionalGetItem::get(
//         //     item_store.as_ref(),
//         //     &mut txn,
//         //     PART_ACTIVITYBAR_SEGKEY.to_segkey_buf(),
//         // );
//         // let activitybar_cache = to_option(
//         //     activitybar_result,
//         //     std::marker::PhantomData::<ActivitybarPartStateEntity>,
//         //     ActivitybarPartCache::from,
//         // )?;

//         let activitybar_defaults = ActivitybarPartDefaults {
//             position: ActivitybarPosition::Default,
//             items: HashMap::new(),
//         };

//         // let activitybar_preferences = ActivitybarPartPreferences {
//         //     position: activitybar_cache
//         //         .as_ref()
//         //         .and_then(|cache| cache.position.clone()),
//         //     items: activitybar_cache.as_ref().map(|cache| cache.items.clone()),
//         // };

//         let activitybar_preferences = ActivitybarPartPreferences {
//             position: None, // HACK: hardcoded for now
//             items: None,    // HACK: hardcoded for now
//         };

//         let activitybar = ActivitybarPart {
//             defaults: activitybar_defaults,
//             preferences: activitybar_preferences,
//             // cache: activitybar_cache,
//         };

//         // ------------------------------------
//         // Sidebar
//         // ------------------------------------

//         // let sidebar_result = TransactionalGetItem::get(
//         //     item_store.as_ref(),
//         //     &mut txn,
//         //     PART_SIDEBAR_SEGKEY.to_segkey_buf(),
//         // );
//         // let sidebar_cache = to_option(
//         //     sidebar_result,
//         //     std::marker::PhantomData::<SidebarPartStateEntity>,
//         //     SidebarPartCache::from,
//         // )?;

//         let sidebar_defaults = SidebarPartDefaults {
//             position: SidebarPosition::Left,
//             size: 200,
//             is_visible: true,
//         };

//         let sidebar_preferences = SidebarPartPreferences {
//             position: None,   // HACK: hardcoded for now
//             is_visible: None, // HACK: hardcoded for now
//         };

//         let sidebar = SidebarPart {
//             defaults: sidebar_defaults,
//             preferences: sidebar_preferences,
//             // cache: sidebar_cache,
//         };

//         // ------------------------------------
//         // Panel
//         // ------------------------------------

//         let panel_defaults = PanelPartDefaults {
//             size: 200,
//             is_visible: true,
//         };

//         let panel_preferences = PanelPartPreferences { is_visible: true };

//         let panel = PanelPart {
//             defaults: panel_defaults,
//             preferences: panel_preferences,
//         };

//         Ok(Self {
//             storage,
//             activitybar,
//             sidebar,
//             panel,
//         })
//     }

//     pub fn layout_info(&self) -> Result<PanelPartInfo> {
//         fn to_option<E, S>(
//             result: Result<AnyValue, DatabaseError>,
//             _: std::marker::PhantomData<E>,
//             convert_fn: impl FnOnce(E) -> S,
//         ) -> Result<Option<S>>
//         where
//             E: DeserializeOwned,
//         {
//             match result {
//                 Ok(value) => {
//                     let entity: E = value.deserialize()?;
//                     Ok(Some(convert_fn(entity)))
//                 }
//                 Err(DatabaseError::NotFound { .. }) => Ok(None),
//                 Err(err) => Err(err.into()),
//             }
//         }

//         let item_store = self.storage.item_store();
//         let mut txn = self.storage.begin_read()?;

//         let activitybar_result = match TransactionalGetItem::get(
//             item_store.as_ref(),
//             &mut txn,
//             PART_ACTIVITYBAR_SEGKEY.to_segkey_buf(),
//         ) {
//             Ok(value) => {
//                 let entity: ActivitybarPartStateEntity = value.deserialize()?;
//                 Some(entity)
//             }
//             Err(DatabaseError::NotFound { .. }) => None,
//             Err(err) => return Err(err.into()),
//         };

//         // let activitybar_cache = to_option(
//         //     activitybar_result,
//         //     std::marker::PhantomData::<ActivitybarPartStateEntity>,
//         //     ActivitybarPartCache::from,
//         // )?;

//         // PanelPartInfo {
//         //     preferred_size: self.panel.defaults.size,
//         //     is_visible: self.panel.preferences.is_visible,
//         // }

//         todo!()
//     }
// }

pub struct LayoutService {
    storage: Arc<dyn WorkspaceStorage>,
}

impl LayoutService {
    pub fn new(storage: Arc<dyn WorkspaceStorage>) -> Self {
        Self { storage }
    }

    pub fn sidebar_state(&self, txn: &mut Transaction) -> Result<SidebarPartStateInfo> {
        // HACK: hardcoded for now
        let preferences = SidebarPartPreferences {
            position: None,
            visible: None,
        };

        let item_store = self.storage.item_store();
        let mut sidebar_cache = TransactionalListByPrefix::list_by_prefix(
            item_store.as_ref(),
            txn,
            PART_SIDEBAR_SEGKEY.as_str().unwrap(),
        )?
        .into_iter()
        .map(|(segkey, value)| (segkey, value))
        .collect::<HashMap<SegKeyBuf, AnyValue>>();

        Ok(SidebarPartStateInfo {
            position: get_from_cache::<SidebarPosition>(
                &mut sidebar_cache,
                PART_SIDEBAR_SEGKEY.join("position"),
            )
            .or(preferences.position)
            .unwrap_or(SIDEBAR_DEFAULTS.position),

            preferred_size: get_from_cache::<usize>(
                &mut sidebar_cache,
                PART_SIDEBAR_SEGKEY.join("size"),
            )
            .unwrap_or(SIDEBAR_DEFAULTS.size),

            visible: get_from_cache::<bool>(
                &mut sidebar_cache,
                PART_SIDEBAR_SEGKEY.join("visible"),
            )
            .or(preferences.visible)
            .unwrap_or(SIDEBAR_DEFAULTS.is_visible),
        })
    }

    pub fn activitybar_state(&self, txn: &mut Transaction) -> Result<ActivitybarPartStateInfo> {
        // HACK: hardcoded for now
        let preferences = ActivitybarPartPreferences {
            position: None,
            items: None,
        };

        let item_store = self.storage.item_store();
        let mut activitybar_cache = TransactionalListByPrefix::list_by_prefix(
            item_store.as_ref(),
            txn,
            PART_ACTIVITYBAR_SEGKEY.as_str().unwrap(),
        )?
        .into_iter()
        .map(|(segkey, value)| (segkey, value))
        .collect::<HashMap<SegKeyBuf, AnyValue>>();

        Ok(ActivitybarPartStateInfo {
            last_active_container_id: get_from_cache::<String>(
                &mut activitybar_cache,
                PART_ACTIVITYBAR_SEGKEY.join("lastActiveContainerId"),
            ),

            position: get_from_cache::<ActivitybarPosition>(
                &mut activitybar_cache,
                PART_ACTIVITYBAR_SEGKEY.join("position"),
            )
            .or(preferences.position)
            .unwrap_or(ACTIVITYBAR_DEFAULTS.position),

            items: ACTIVITYBAR_DEFAULTS
                .items
                .iter()
                .map(|default_item| {
                    let container_preferences = preferences
                        .items
                        .as_ref()
                        .and_then(|items| items.iter().find(|item| item.id == default_item.id));
                    let container_segkey = PART_ACTIVITYBAR_SEGKEY
                        .join("container")
                        .join(default_item.id);

                    ActivitybarItemStateInfo {
                        id: default_item.id.to_string(),

                        order: get_from_cache::<usize>(
                            &mut activitybar_cache,
                            container_segkey.join("order"),
                        )
                        .or(container_preferences.and_then(|p| p.order))
                        .unwrap_or(default_item.order),

                        visible: get_from_cache::<bool>(
                            &mut activitybar_cache,
                            container_segkey.join("visible"),
                        )
                        .or(container_preferences.and_then(|p| p.visible))
                        .unwrap_or(default_item.visible),
                    }
                })
                .collect(),
        })
    }

    pub fn panel_state(&self, txn: &mut Transaction) -> Result<PanelPartStateInfo> {
        // HACK: hardcoded for now
        let preferences = PanelPartPreferences { visible: None };

        let item_store = self.storage.item_store();
        let mut panel_cache = TransactionalListByPrefix::list_by_prefix(
            item_store.as_ref(),
            txn,
            PART_PANEL_SEGKEY.as_str().unwrap(),
        )?
        .into_iter()
        .map(|(segkey, value)| (segkey, value))
        .collect::<HashMap<SegKeyBuf, AnyValue>>();

        Ok(PanelPartStateInfo {
            preferred_size: get_from_cache::<usize>(
                &mut panel_cache,
                PART_PANEL_SEGKEY.join("size"),
            )
            .unwrap_or(PANEL_DEFAULTS.size),

            visible: get_from_cache::<bool>(&mut panel_cache, PART_PANEL_SEGKEY.join("visible"))
                .or(preferences.visible)
                .unwrap_or(PANEL_DEFAULTS.is_visible),
        })
    }

    // FIXME: should not be `Option`. Its a temporary solution since we cannot have defaults for editor part now.
    pub fn editor_state(&self, txn: &mut Transaction) -> Result<Option<EditorPartStateInfo>> {
        // HACK: hardcoded for now
        let _preferences = EditorPartPreferences {};

        let item_store = self.storage.item_store();
        let mut editor_cache = TransactionalListByPrefix::list_by_prefix(
            item_store.as_ref(),
            txn,
            PART_EDITOR_SEGKEY.as_str().unwrap(),
        )?
        .into_iter()
        .map(|(segkey, value)| (segkey, value))
        .collect::<HashMap<SegKeyBuf, AnyValue>>();

        let grid =
            get_from_cache::<EditorGridState>(&mut editor_cache, PART_EDITOR_SEGKEY.join("grid"));
        let grid = if let Some(grid) = grid {
            grid
        } else {
            // Will use a default grid here later
            return Ok(None);
        };

        let panels = get_from_cache::<HashMap<String, EditorPanelState>>(
            &mut editor_cache,
            PART_EDITOR_SEGKEY.join("panels"),
        )
        .unwrap_or_default();

        let active_group =
            get_from_cache::<String>(&mut editor_cache, PART_EDITOR_SEGKEY.join("activeGroup"));

        Ok(Some(EditorPartStateInfo {
            grid,
            panels,
            active_group,
        }))
    }
}

fn get_from_cache<T: DeserializeOwned>(
    cache: &mut HashMap<SegKeyBuf, AnyValue>,
    key: SegKeyBuf,
) -> Option<T> {
    cache
        .remove(&key)
        .and_then(|raw_value| match raw_value.deserialize() {
            Ok(entity) => Some(entity),
            Err(err) => {
                println!("Error deserializing value: {:?}", err);
                None
            }
        })
}
