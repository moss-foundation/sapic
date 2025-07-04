use anyhow::Result;
use moss_applib::ServiceMarker;
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
            ActivitybarItemStateInfo, ActivitybarPartStateInfo, EditorPartStateInfo,
            PanelPartStateInfo, SidebarPartStateInfo,
        },
    },
    services::storage_service::StorageService,
    storage::{
        entities::state_store::{EditorGridStateEntity, EditorPanelStateEntity},
        segments::{
            LAYOUT_ACTIVITYBAR_SEGKEY, LAYOUT_EDITOR_SEGKEY, LAYOUT_PANEL_SEGKEY,
            LAYOUT_SIDEBAR_SEGKEY,
        },
    },
};
// ------------------------------------
// Activitybar
// ------------------------------------

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

// ------------------------------------
// Sidebar
// ------------------------------------

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

// ------------------------------------
// Panel
// ------------------------------------

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
    is_visible: false,
};

// ------------------------------------
// Editor
// ------------------------------------

#[derive(Debug)]
pub struct EditorPartDefaults {}

#[derive(Debug)]
pub struct EditorPartPreferences {}

const _EDITOR_DEFAULTS: EditorPartDefaults = EditorPartDefaults {};

pub struct LayoutService {
    storage: Arc<StorageService>, // TODO: should be a trait
}

impl ServiceMarker for LayoutService {}

impl LayoutService {
    pub fn new(storage: Arc<StorageService>) -> Self {
        Self { storage }
    }

    pub fn put_editor_layout_state(&self, state: EditorPartStateInfo) -> Result<()> {
        let editor_grid = EditorGridStateEntity::from(state.grid);
        let panels = state
            .panels
            .into_iter()
            .map(|(key, panel)| (key, panel.into()))
            .collect::<HashMap<String, EditorPanelStateEntity>>();

        self.storage
            .put_editor_layout(editor_grid, panels, state.active_group)?;

        Ok(())
    }

    pub fn put_sidebar_layout_state(&self, state: SidebarPartStateInfo) -> Result<()> {
        self.storage
            .put_sidebar_layout(state.position, state.size, state.visible)?;

        Ok(())
    }

    pub fn put_panel_layout_state(&self, state: PanelPartStateInfo) -> Result<()> {
        self.storage.put_panel_layout(state.size, state.visible)?;

        Ok(())
    }

    pub fn put_activitybar_layout_state(&self, state: ActivitybarPartStateInfo) -> Result<()> {
        self.storage
            .put_activitybar_layout(state.last_active_container_id, state.position)?;

        Ok(())
    }

    // HACK: cache as a parameter here is a temporary solution
    pub fn get_sidebar_layout_state(
        &self,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<SidebarPartStateInfo> {
        // HACK: hardcoded for now
        let preferences = SidebarPartPreferences {
            position: None,
            visible: None,
        };

        Ok(SidebarPartStateInfo {
            position: get_from_cache::<SidebarPosition>(
                cache,
                LAYOUT_SIDEBAR_SEGKEY.join("position"),
            )
            .or(preferences.position)
            .unwrap_or(SIDEBAR_DEFAULTS.position),

            size: get_from_cache::<usize>(cache, LAYOUT_SIDEBAR_SEGKEY.join("size"))
                .unwrap_or(SIDEBAR_DEFAULTS.size),

            visible: get_from_cache::<bool>(cache, LAYOUT_SIDEBAR_SEGKEY.join("visible"))
                .or(preferences.visible)
                .unwrap_or(SIDEBAR_DEFAULTS.is_visible),
        })
    }

    // HACK: cache as a parameter here is a temporary solution
    pub fn get_activitybar_layout_state(
        &self,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<ActivitybarPartStateInfo> {
        // HACK: hardcoded for now
        let preferences = ActivitybarPartPreferences {
            position: None,
            items: None,
        };

        Ok(ActivitybarPartStateInfo {
            last_active_container_id: get_from_cache::<String>(
                cache,
                LAYOUT_ACTIVITYBAR_SEGKEY.join("lastActiveContainerId"),
            )
            .or_else(|| Some(TREE_VIEW_GROUP_COLLECTIONS.to_string())),

            position: get_from_cache::<ActivitybarPosition>(
                cache,
                LAYOUT_ACTIVITYBAR_SEGKEY.join("position"),
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
                    let container_segkey = LAYOUT_ACTIVITYBAR_SEGKEY
                        .join("container")
                        .join(default_item.id);

                    ActivitybarItemStateInfo {
                        id: default_item.id.to_string(),

                        order: get_from_cache::<usize>(cache, container_segkey.join("order"))
                            .or(container_preferences.and_then(|p| p.order))
                            .unwrap_or(default_item.order),

                        visible: get_from_cache::<bool>(cache, container_segkey.join("visible"))
                            .or(container_preferences.and_then(|p| p.visible))
                            .unwrap_or(default_item.visible),
                    }
                })
                .collect(),
        })
    }

    pub fn get_panel_layout_state(
        &self,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<PanelPartStateInfo> {
        // HACK: hardcoded for now
        let preferences = PanelPartPreferences { visible: None };

        Ok(PanelPartStateInfo {
            size: get_from_cache::<usize>(cache, LAYOUT_PANEL_SEGKEY.join("size"))
                .unwrap_or(PANEL_DEFAULTS.size),

            visible: get_from_cache::<bool>(cache, LAYOUT_PANEL_SEGKEY.join("visible"))
                .or(preferences.visible)
                .unwrap_or(PANEL_DEFAULTS.is_visible),
        })
    }

    // FIXME: should not be `Option`. Its a temporary solution since we cannot have defaults for editor part now.
    pub fn get_editor_layout_state(
        &self,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<Option<EditorPartStateInfo>> {
        // HACK: hardcoded for now
        let _preferences = EditorPartPreferences {};

        let grid =
            get_from_cache::<EditorGridStateEntity>(cache, LAYOUT_EDITOR_SEGKEY.join("grid"));
        let grid = if let Some(grid) = grid {
            grid
        } else {
            // Will use a default grid here later
            return Ok(None);
        };

        let panels = get_from_cache::<HashMap<String, EditorPanelStateEntity>>(
            cache,
            LAYOUT_EDITOR_SEGKEY.join("panels"),
        )
        .unwrap_or_default();

        let active_group =
            get_from_cache::<String>(cache, LAYOUT_EDITOR_SEGKEY.join("activeGroup"));

        Ok(Some(EditorPartStateInfo {
            grid: grid.into(),
            panels: panels
                .into_iter()
                .map(|(key, panel)| (key, panel.into()))
                .collect(),
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
