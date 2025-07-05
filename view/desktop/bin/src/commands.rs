mod app;
mod collection;
mod workspace;

pub use app::*;
pub use collection::*;
pub use workspace::*;

use anyhow::Context as _;
use moss_api::{TauriError, TauriResult};
use moss_app::{app::App, services::workspace_service::WorkspaceService};
use moss_collection::Collection;
use moss_common::api::OperationOptionExt;
use moss_workspace::{
    Workspace, context::WorkspaceContext, services::collection_service::CollectionService,
};
use std::sync::Arc;
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
        let app_handle = app.app_handle();
        let (workspace, _ctx) = app
            .service::<WorkspaceService<R>>()
            .workspace_with_context(app_handle)
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

pub(super) async fn with_workspace_timeout<R, T, F, Fut>(
    app: State<'_, App<R>>,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: TauriRuntime,
    F: FnOnce(WorkspaceContext<R>, Arc<Workspace<R>>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = TauriResult<T>> + Send + 'static,
{
    moss_api::with_timeout(options, async move {
        let app_handle = app.app_handle();
        let (workspace, ctx) = app
            .service::<WorkspaceService<R>>()
            .workspace_with_context(app_handle)
            .await
            .map_err_as_failed_precondition("No active workspace")?;

        f(ctx, workspace).await
    })
    .await
    .map_err(|_| TauriError::Timeout)?
}
