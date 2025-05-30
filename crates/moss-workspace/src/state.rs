use anyhow::Result;
use moss_db::{DatabaseError, primitives::AnyValue};
use moss_storage::{
    WorkspaceStorage,
    storage::operations::TransactionalGetItem,
    workspace_storage::entities::state_store_entities::{
        ActivitybarPartStateEntity, SidebarPartStateEntity,
    },
};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, sync::Arc};

use crate::{
    models::primitives::{ActivitybarPosition, SidebarPosition},
    storage::segments::{PART_ACTIVITYBAR_SEGKEY, PART_SIDEBAR_SEGKEY},
};

// ------------------------------------
// Activitybar
// ------------------------------------

#[derive(Debug, Clone)]
pub struct ActivitybarItem {
    pub order: Option<usize>,
    pub visible: Option<bool>,
}

#[derive(Debug)]
pub struct ActivitybarPartPreferences {
    pub position: Option<ActivitybarPosition>,
    pub items: Option<HashMap<String, ActivitybarItem>>,
}

#[derive(Debug)]
pub struct ActivitybarPartDefaults {
    pub position: ActivitybarPosition,
    pub items: HashMap<String, ActivitybarItem>,
}

#[derive(Debug)]
pub struct ActivitybarPartCache {
    pub last_active_tree_view_item_id: Option<String>,

    // HACK: this is a temporary solution to store the position of the activitybar and items order,
    // as part of the workspace state. We should store it as a user preference not a workspace state.
    position: Option<ActivitybarPosition>,
    items: HashMap<String, ActivitybarItem>,
}

impl From<ActivitybarPartStateEntity> for ActivitybarPartCache {
    fn from(entity: ActivitybarPartStateEntity) -> Self {
        Self {
            last_active_tree_view_item_id: entity.last_active_tree_view_item,
            position: None,        // TODO: extract from entity if available
            items: HashMap::new(), // TODO: extract from entity if available
        }
    }
}

#[derive(Debug)]
pub struct ActivitybarPart {
    pub defaults: ActivitybarPartDefaults,
    pub preferences: ActivitybarPartPreferences,
    pub cache: Option<ActivitybarPartCache>,
}

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
    is_visible: Option<bool>,
}

#[derive(Debug)]
pub struct SidebarPartCache {
    preferred_size: usize,
    is_visible: bool,
}

impl From<SidebarPartStateEntity> for SidebarPartCache {
    fn from(entity: SidebarPartStateEntity) -> Self {
        Self {
            preferred_size: entity.preferred_size,
            is_visible: entity.is_visible,
        }
    }
}

#[derive(Debug)]
pub struct SidebarPart {
    pub defaults: SidebarPartDefaults,
    pub preferences: SidebarPartPreferences,
    pub cache: Option<SidebarPartCache>,
}

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
    is_visible: bool,
}

#[derive(Debug)]
pub struct PanelPartCache {
    preferred_size: usize,
    is_visible: bool,
}

#[derive(Debug)]
pub struct PanelPart {
    pub defaults: PanelPartDefaults,
    pub preferences: PanelPartPreferences,
    pub cache: Option<PanelPartCache>,
}

// ------------------------------------
// Editor
// ------------------------------------

#[derive(Debug)]
pub struct EditorPartDefaults {}

#[derive(Debug)]
pub struct EditorPartPreferences {}

#[derive(Debug)]
pub struct EditorPartCache {}

#[derive(Debug)]
pub struct EditorPart {
    pub defaults: EditorPartDefaults,
    pub preferences: EditorPartPreferences,
    pub cache: Option<EditorPartCache>,
}

// ------------------------------------

pub struct LayoutService {
    storage: Arc<dyn WorkspaceStorage>, // TODO: replace with just a store when we have a way to begin a transaction from the store
    activitybar: ActivitybarPart,
    sidebar: SidebarPart,
}

impl LayoutService {
    pub fn new(storage: Arc<dyn WorkspaceStorage>) -> Result<Self> {
        let item_store = storage.item_store();
        let mut txn = storage.begin_read()?;

        fn to_option<E, S>(
            result: Result<AnyValue, DatabaseError>,
            _: std::marker::PhantomData<E>,
            convert_fn: impl FnOnce(E) -> S,
        ) -> Result<Option<S>>
        where
            E: DeserializeOwned,
        {
            match result {
                Ok(value) => {
                    let entity: E = value.deserialize()?;
                    Ok(Some(convert_fn(entity)))
                }
                Err(DatabaseError::NotFound { .. }) => Ok(None),
                Err(err) => Err(err.into()),
            }
        }

        // ------------------------------------
        // Activitybar
        // ------------------------------------

        let activitybar_result = TransactionalGetItem::get(
            item_store.as_ref(),
            &mut txn,
            PART_ACTIVITYBAR_SEGKEY.to_segkey_buf(),
        );
        let activitybar_cache = to_option(
            activitybar_result,
            std::marker::PhantomData::<ActivitybarPartStateEntity>,
            ActivitybarPartCache::from,
        )?;

        let activitybar_defaults = ActivitybarPartDefaults {
            position: ActivitybarPosition::Default,
            items: HashMap::new(),
        };

        let activitybar_preferences = ActivitybarPartPreferences {
            position: activitybar_cache
                .as_ref()
                .and_then(|cache| cache.position.clone()),
            items: activitybar_cache.as_ref().map(|cache| cache.items.clone()),
        };

        let activitybar = ActivitybarPart {
            defaults: activitybar_defaults,
            preferences: activitybar_preferences,
            cache: activitybar_cache,
        };

        // ------------------------------------
        // Sidebar
        // ------------------------------------

        let sidebar_result = TransactionalGetItem::get(
            item_store.as_ref(),
            &mut txn,
            PART_SIDEBAR_SEGKEY.to_segkey_buf(),
        );
        let sidebar_cache = to_option(
            sidebar_result,
            std::marker::PhantomData::<SidebarPartStateEntity>,
            SidebarPartCache::from,
        )?;

        let sidebar_defaults = SidebarPartDefaults {
            position: SidebarPosition::Left,
            size: 200,
            is_visible: true,
        };

        let sidebar_preferences = SidebarPartPreferences {
            position: None,   // HACK: hardcoded for now
            is_visible: None, // HACK: hardcoded for now
        };

        let sidebar = SidebarPart {
            defaults: sidebar_defaults,
            preferences: sidebar_preferences,
            cache: sidebar_cache,
        };

        // ------------------------------------
        // Panel
        // ------------------------------------

        Ok(Self {
            storage,
            activitybar,
            sidebar,
        })
    }
}
