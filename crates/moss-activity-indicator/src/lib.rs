mod constants;
pub mod handle;
pub mod models;

use anyhow::Result;
use constants::ACTIVITY_INDICATOR_CHANNEL;
use handle::ActivityHandle;
use models::events::ActivityEvent;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tauri::{AppHandle, Emitter, Runtime as TauriRuntime};

pub struct ActivityIndicator<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    next_id: Arc<AtomicUsize>,
}

impl<R: TauriRuntime> Clone for ActivityIndicator<R> {
    fn clone(&self) -> Self {
        Self {
            app_handle: self.app_handle.clone(),
            next_id: Arc::clone(&self.next_id),
        }
    }
}

impl<R: TauriRuntime> ActivityIndicator<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self {
            app_handle,
            next_id: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn emit_oneshot(
        &self,
        activity_id: &str,
        title: String,
        detail: Option<String>,
    ) -> Result<()> {
        self.app_handle.emit(
            ACTIVITY_INDICATOR_CHANNEL,
            ActivityEvent::Oneshot {
                id: self.next_id.fetch_add(1, Ordering::SeqCst),
                activity_id,
                title,
                detail,
            },
        )?;

        Ok(())
    }

    pub fn emit_continual<'a>(
        &'a self,
        activity_id: &'a str,
        title: String,
        detail: Option<String>,
    ) -> Result<ActivityHandle<'a, R>> {
        self.app_handle.emit(
            ACTIVITY_INDICATOR_CHANNEL,
            ActivityEvent::Start {
                activity_id,
                title,
                detail,
                id: self.next_id.fetch_add(1, Ordering::SeqCst),
            },
        )?;

        Ok(ActivityHandle::new(
            activity_id,
            self.app_handle.clone(),
            Arc::clone(&self.next_id),
        ))
    }
}
