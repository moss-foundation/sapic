use anyhow::Result;
use serde::Serialize;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tauri::{AppHandle, Emitter, Runtime as TauriRuntime};
use ts_rs::TS;

const ACTIVITY_INDICATOR_CHANNEL: &str = "workbench://activity-indicator";

#[derive(Serialize, Clone, TS)]
#[ts(export, export_to = "types.ts")]
#[serde(rename_all = "camelCase")]
enum ActivityEvent<'a> {
    /// This event is used when the activity is a one-time event
    /// and we don't want to track its progress.
    #[serde(rename_all = "camelCase")]
    Oneshot {
        id: u64,
        activity_id: &'a str,
        title: String,
        #[ts(optional)]
        detail: Option<String>,
    },
    /// This event is used when the activity is a long-running event
    /// and we want to track its progress, like indexing, scanning, etc.
    #[serde(rename_all = "camelCase")]
    Start {
        id: u64,
        activity_id: &'a str,
        title: String,
        #[ts(optional)]
        detail: Option<String>,
    },
    /// This event is used to update the progress of a long-running activity,
    /// like updating the progress of an indexer, scanner, etc.
    #[serde(rename_all = "camelCase")]
    Progress {
        id: u64,
        activity_id: &'a str,
        #[ts(optional)]
        detail: Option<String>,
    },
    /// This event is used to notify the frontend that the long-running activity
    /// is finished and the activity indicator should be hidden.
    #[serde(rename_all = "camelCase")]
    Finish { id: u64, activity_id: &'a str },
}

pub struct ActivityHandle<'a, R: TauriRuntime> {
    pub activity_id: &'a str,
    pub app_handle: AppHandle<R>,

    next_id: Arc<AtomicU64>,
}

impl<'a, R: TauriRuntime> ActivityHandle<'a, R> {
    pub fn new(activity_id: &'a str, app_handle: AppHandle<R>, next_id: Arc<AtomicU64>) -> Self {
        Self {
            activity_id,
            app_handle,
            next_id,
        }
    }

    pub fn emit_progress(&self, detail: Option<String>) -> Result<()> {
        self.app_handle.emit(
            ACTIVITY_INDICATOR_CHANNEL,
            ActivityEvent::Progress {
                id: self.next_id.fetch_add(1, Ordering::SeqCst),
                activity_id: self.activity_id,
                detail,
            },
        )?;
        Ok(())
    }

    pub fn emit_finish(&self) -> Result<()> {
        self.app_handle.emit(
            ACTIVITY_INDICATOR_CHANNEL,
            ActivityEvent::Finish {
                id: self.next_id.fetch_add(1, Ordering::SeqCst),
                activity_id: self.activity_id,
            },
        )?;

        Ok(())
    }
}

pub struct ActivityIndicator<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    next_id: Arc<AtomicU64>,
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
            next_id: Arc::new(AtomicU64::new(0)),
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
    ) -> Result<ActivityHandle<R>> {
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
