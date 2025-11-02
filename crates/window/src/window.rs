pub mod logging; // HACK: temporary public
pub mod session; // HACK: temporary public
mod workspace;

pub mod types;

use moss_applib::context::Canceller;
use std::{collections::HashMap, sync::Arc};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::RwLock;

pub struct Window<R: TauriRuntime> {
    app_handle: AppHandle<R>,

    // Store cancellers by the id of API requests
    tracked_cancellations: Arc<RwLock<HashMap<String, Canceller>>>,
}

impl<R: TauriRuntime> Window<R> {
    pub async fn track_cancellation(&self, request_id: &str, canceller: Canceller) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.insert(request_id.to_string(), canceller);
    }

    pub async fn release_cancellation(&self, request_id: &str) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.remove(request_id);
    }
}
