// TODO: This should be moved to the runtime after the `AppDelegate` is extracted from the system layer of the application (right now, it's causing circular dependencies).

pub mod handle;

use handle::ActivityHandle;
use moss_applib::TauriResultExt;
use sapic_base::{
    language::types::LocalizedString, notification::types::primitives::NotificationLocation,
};
use sapic_ipc::contracts::notification::ActivityEvent;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tauri::{AppHandle, Emitter, Runtime as TauriRuntime};

pub(crate) mod constants {
    use moss_bindingutils::const_export;

    /// @category Constant
    #[const_export(export_to = "constants.ts")]
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
        title: LocalizedString,
        detail: Option<LocalizedString>,
    },
    Notification {
        activity_id: &'a str,
        title: LocalizedString,
        detail: Option<LocalizedString>,
    },
    Toast {
        activity_id: &'a str,
        title: LocalizedString,
        detail: Option<LocalizedString>,
    },
}

impl<'a> ToLocation<'a> {
    fn location(&self) -> NotificationLocation {
        match self {
            ToLocation::Window { .. } => NotificationLocation::Window,
            ToLocation::Notification { .. } => NotificationLocation::Notification,
            ToLocation::Toast { .. } => NotificationLocation::Toast,
        }
    }

    fn activity_id(&self) -> &'a str {
        match self {
            ToLocation::Window { activity_id, .. } => activity_id,
            ToLocation::Notification { activity_id, .. } => activity_id,
            ToLocation::Toast { activity_id, .. } => activity_id,
        }
    }

    fn title(&self) -> LocalizedString {
        match self {
            ToLocation::Window { title, .. } => title.clone(),
            ToLocation::Notification { title, .. } => title.clone(),
            ToLocation::Toast { title, .. } => title.clone(),
        }
    }

    fn detail(&self) -> Option<LocalizedString> {
        match self {
            ToLocation::Window { detail, .. } => detail.clone(),
            ToLocation::Notification { detail, .. } => detail.clone(),
            ToLocation::Toast { detail, .. } => detail.clone(),
        }
    }
}

/// A broadcaster for activity events.
///
/// Use `AppActivityBroadcaster` to emit activity events to the frontend.
pub struct AppActivityBroadcaster<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    next_id: Arc<AtomicUsize>,
}

impl<R: TauriRuntime> Clone for AppActivityBroadcaster<R> {
    fn clone(&self) -> Self {
        Self {
            app_handle: self.app_handle.clone(),
            next_id: Arc::clone(&self.next_id),
        }
    }
}

impl<R: TauriRuntime> AppActivityBroadcaster<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self {
            app_handle,
            next_id: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn emit_oneshot(&self, to: ToLocation<'_>) -> joinerror::Result<()> {
        self.app_handle
            .emit(
                constants::CHANNEL,
                ActivityEvent::Oneshot {
                    id: self.next_id.fetch_add(1, Ordering::SeqCst),
                    activity_id: to.activity_id(),
                    title: to.title(),
                    detail: to.detail(),
                    location: to.location(),
                },
            )
            .join_err_bare()
    }

    pub fn emit_continual<'a>(
        &'a self,
        to: ToLocation<'a>,
    ) -> joinerror::Result<ActivityHandle<'a, R>> {
        let activity_id = to.activity_id();
        self.app_handle
            .emit(
                constants::CHANNEL,
                ActivityEvent::Start {
                    id: self.next_id.fetch_add(1, Ordering::SeqCst),
                    activity_id: activity_id,
                    title: to.title(),
                    detail: to.detail(),
                    location: to.location(),
                },
            )
            .join_err_bare()?;

        Ok(ActivityHandle::new(
            activity_id,
            self.app_handle.clone(),
            Arc::clone(&self.next_id),
        ))
    }
}
