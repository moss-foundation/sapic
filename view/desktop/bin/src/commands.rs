mod app;
mod collection;
mod workspace;

use std::sync::Arc;

pub use app::*;
pub use collection::*;
pub use workspace::*;

use anyhow::Context as _;
use moss_api::{TauriError, TauriResult};
use moss_app::app::App;
use moss_collection::Collection;
use moss_common::api::OperationOptionExt;
use moss_workspace::services::collection_service::CollectionService;
use tauri::{Runtime as TauriRuntime, State};
use uuid::Uuid;

pub(super) type Options = Option<moss_api::models::types::Options>;

pub(super) async fn with_collection_timeout<R, T, F, Fut>(
    app: State<'_, App<R>>,
    id: Uuid,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: TauriRuntime,
    F: FnOnce(Arc<Collection>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = TauriResult<T>> + Send + 'static,
{
    moss_api::with_timeout(options, async move {
        let (workspace, _ctx) = app
            .workspace()
            .await
            .map_err_as_failed_precondition("No active workspace")?;

        let collection = workspace
            .service::<CollectionService>()
            .collection(id)
            .await
            .context("Collection not found")?;

        f(collection).await
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}
