pub mod handle;
pub mod models;

use anyhow::Result;
use handle::ActivityHandle;
use models::events::ActivityEvent;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tauri::{AppHandle, Emitter, Runtime as TauriRuntime};

use crate::models::primitives::Location;

pub(crate) mod constants {
    // ######################################################################
    // ###                                                                ###
    // ### !!! PLEASE UPDATE THE TYPESCRIPT CONSTANTS IN constants.ts !!! ###
    // ###                                                                ###
    // ######################################################################

    pub(crate) const CHANNEL: &str = "app://activity";
}

/// Represents where progress or transient messages are displayed.
///
/// Use `ToLocation` to route UI messages to a specific surface within the
/// application shell. Variants are ordered from most global to most
/// ephemeral.
///
/// For more information, see [Location].
pub enum ToLocation<'a> {
    Window {
        activity_id: &'a str,
        title: String,
        detail: Option<String>,
    },
    Notification {
        activity_id: &'a str,
        title: String,
        detail: Option<String>,
    },
    Toast {
        activity_id: &'a str,
        title: String,
        detail: Option<String>,
    },
}

impl<'a> ToLocation<'a> {
    fn location(&self) -> Location {
        match self {
            ToLocation::Window { .. } => Location::Window,
            ToLocation::Notification { .. } => Location::Notification,
            ToLocation::Toast { .. } => Location::Toast,
        }
    }

    fn activity_id(&self) -> &'a str {
        match self {
            ToLocation::Window { activity_id, .. } => activity_id,
            ToLocation::Notification { activity_id, .. } => activity_id,
            ToLocation::Toast { activity_id, .. } => activity_id,
        }
    }

    fn title(&self) -> String {
        match self {
            ToLocation::Window { title, .. } => title.clone(),
            ToLocation::Notification { title, .. } => title.clone(),
            ToLocation::Toast { title, .. } => title.clone(),
        }
    }

    fn detail(&self) -> Option<String> {
        match self {
            ToLocation::Window { detail, .. } => detail.clone(),
            ToLocation::Notification { detail, .. } => detail.clone(),
            ToLocation::Toast { detail, .. } => detail.clone(),
        }
    }
}

/// A broadcaster for activity events.
///
/// Use `ActivityBroadcaster` to emit activity events to the frontend.
pub struct ActivityBroadcaster<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    next_id: Arc<AtomicUsize>,
}

impl<R: TauriRuntime> Clone for ActivityBroadcaster<R> {
    fn clone(&self) -> Self {
        Self {
            app_handle: self.app_handle.clone(),
            next_id: Arc::clone(&self.next_id),
        }
    }
}

impl<R: TauriRuntime> ActivityBroadcaster<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self {
            app_handle,
            next_id: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn emit_oneshot(&self, to: ToLocation<'_>) -> Result<()> {
        self.app_handle.emit(
            constants::CHANNEL,
            ActivityEvent::Oneshot {
                id: self.next_id.fetch_add(1, Ordering::SeqCst),
                activity_id: to.activity_id(),
                title: to.title(),
                detail: to.detail(),
                location: to.location(),
            },
        )?;

        Ok(())
    }

    pub fn emit_continual<'a>(&'a self, to: ToLocation<'a>) -> Result<ActivityHandle<'a, R>> {
        let activity_id = to.activity_id();
        self.app_handle.emit(
            constants::CHANNEL,
            ActivityEvent::Start {
                id: self.next_id.fetch_add(1, Ordering::SeqCst),
                activity_id: activity_id,
                title: to.title(),
                detail: to.detail(),
                location: to.location(),
            },
        )?;

        Ok(ActivityHandle::new(
            activity_id,
            self.app_handle.clone(),
            Arc::clone(&self.next_id),
        ))
    }
}
