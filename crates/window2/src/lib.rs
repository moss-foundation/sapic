use async_trait::async_trait;
use moss_applib::context::Canceller;

pub mod constants {
    pub const MIN_WINDOW_WIDTH: f64 = 800.0;
    pub const MIN_WINDOW_HEIGHT: f64 = 600.0;
}

pub mod defaults {
    pub const DEFAULT_WINDOW_POSITION_X: f64 = 100.0;
    pub const DEFAULT_WINDOW_POSITION_Y: f64 = 100.0;
}

#[async_trait]
pub trait WindowApi: Send + Sync + 'static {
    async fn track_cancellation(&self, request_id: &str, canceller: Canceller) -> ();
    async fn release_cancellation(&self, request_id: &str) -> ();
}
