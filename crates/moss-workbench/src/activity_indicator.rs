use anyhow::Result;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Runtime as TauriRuntime};
use ts_rs::TS;

const ACTIVITY_INDICATOR_CHANNEL: &str = "workbench://activity-indicator";

#[derive(Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
enum ActivityEvent<'a> {
    /// This event is used when the activity is a one-time event
    /// and we don't want to track its progress.
    Oneshot {
        activity_id: &'a str,
        title: String,
        detail: String,
    },
    /// This event is used when the activity is a long-running event
    /// and we want to track its progress, like indexing, scanning, etc.
    Start {
        activity_id: &'a str,
        title: String,
        detail: String,
        initial_progress: Option<u8>, // 0-100
    },
    /// This event is used to update the progress of a long-running activity,
    /// like updating the progress of an indexer, scanner, etc.
    Progress {
        activity_id: &'a str,
        detail: String,
        progress: u8, // 0-100
    },
    /// This event is used to notify the frontend that the long-running activity
    /// is finished and the activity indicator should be hidden.
    Finish { activity_id: &'a str },
}

pub struct ActivityHandle<'a, R: TauriRuntime> {
    pub activity_id: &'a str,
    pub app_handle: AppHandle<R>,
}

impl<'a, R: TauriRuntime> ActivityHandle<'a, R> {
    pub fn new(activity_id: &'a str, app_handle: AppHandle<R>) -> Self {
        Self {
            activity_id,
            app_handle,
        }
    }

    pub fn emit_progress(&self, progress: u8, detail: String) -> Result<()> {
        self.app_handle.emit(
            ACTIVITY_INDICATOR_CHANNEL,
            ActivityEvent::Progress {
                activity_id: self.activity_id,
                detail,
                progress,
            },
        )?;
        Ok(())
    }

    pub fn emit_finish(&self) -> Result<()> {
        self.app_handle.emit(
            ACTIVITY_INDICATOR_CHANNEL,
            ActivityEvent::Finish {
                activity_id: self.activity_id,
            },
        )?;

        Ok(())
    }
}

pub struct ActivityIndicator<R: TauriRuntime> {
    app_handle: AppHandle<R>,
}

impl<R: TauriRuntime> Clone for ActivityIndicator<R> {
    fn clone(&self) -> Self {
        Self {
            app_handle: self.app_handle.clone(),
        }
    }
}

impl<R: TauriRuntime> ActivityIndicator<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self { app_handle }
    }

    pub fn emit_oneshot(&self, activity_id: &str, title: String, detail: String) -> Result<()> {
        self.app_handle.emit(
            ACTIVITY_INDICATOR_CHANNEL,
            ActivityEvent::Oneshot {
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
        detail: String,
        initial_progress: Option<u8>,
    ) -> Result<ActivityHandle<R>> {
        self.app_handle.emit(
            ACTIVITY_INDICATOR_CHANNEL,
            ActivityEvent::Start {
                activity_id,
                title,
                detail,
                initial_progress,
            },
        )?;

        Ok(ActivityHandle::new(activity_id, self.app_handle.clone()))
    }
}
