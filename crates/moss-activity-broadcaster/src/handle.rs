use moss_applib::TauriResultExt;
use sapic_ipc::contracts::notification::ActivityEvent;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tauri::{AppHandle, Emitter, Runtime as TauriRuntime};

use crate::constants;

pub struct ActivityHandle<'a, R: TauriRuntime> {
    pub activity_id: &'a str,
    pub app_handle: AppHandle<R>,

    next_id: Arc<AtomicUsize>,
}

impl<'a, R: TauriRuntime> ActivityHandle<'a, R> {
    pub fn new(activity_id: &'a str, app_handle: AppHandle<R>, next_id: Arc<AtomicUsize>) -> Self {
        Self {
            activity_id,
            app_handle,
            next_id,
        }
    }

    pub fn emit_progress(&self, detail: Option<String>) -> joinerror::Result<()> {
        self.app_handle
            .emit(
                constants::CHANNEL,
                ActivityEvent::Progress {
                    id: self.next_id.fetch_add(1, Ordering::SeqCst),
                    activity_id: self.activity_id,
                    detail,
                },
            )
            .join_err_bare()
    }

    pub fn emit_finish(&self) -> joinerror::Result<()> {
        self.app_handle
            .emit(
                constants::CHANNEL,
                ActivityEvent::Finish {
                    id: self.next_id.fetch_add(1, Ordering::SeqCst),
                    activity_id: self.activity_id,
                },
            )
            .join_err_bare()
    }
}
