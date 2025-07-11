mod app;
mod collection;
mod workspace;

pub use app::*;
pub use collection::*;
use moss_applib::ctx::{self, Reason};
pub use workspace::*;

use anyhow::Context as _;
use moss_api::{TauriError, TauriResult};
use moss_app::{app::App, services::workspace_service::WorkspaceService};
use moss_collection::Collection;
use moss_common::api::OperationOptionExt;
use moss_workspace::{
    Workspace, context::WorkspaceContext, models::primitives::CollectionId,
    services::DynCollectionService,
};
use std::{sync::Arc, time::Duration};
use tauri::{Runtime as TauriRuntime, State};

pub(super) type Options = Option<moss_api::models::types::Options>;

pub(super) async fn collection_with_context<R, T, F, Fut>(
    root_ctx: ctx::Context,
    app: State<'_, App<R>>,
    id: CollectionId,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: TauriRuntime,
    F: FnOnce(&mut ctx::MutableContext, Arc<Collection>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = TauriResult<T>> + Send + 'static,
{
    let timeout = options
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(Duration::from_secs(30));

    let mut ctx = ctx::MutableContext::new(&root_ctx);
    ctx.with_timeout(timeout);

    let app_handle = app.handle();
    let (workspace, _child_ctx) = app
        .service::<WorkspaceService<R>>()
        .workspace_with_context(app_handle)
        .await
        .map_err_as_failed_precondition("No active workspace")?;

    let collection = workspace
        .service::<DynCollectionService>()
        .collection(&id)
        .await
        .context("Collection not found")?;

    let res = f(&mut ctx, collection).await?;

    if let Some(Reason::Timedout) = ctx.done() {
        return Err(TauriError::Timeout);
    }

    Ok(res)
}

pub(super) async fn with_collection_timeout<R, T, F, Fut>(
    app: State<'_, App<R>>,
    id: CollectionId,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: TauriRuntime,
    F: FnOnce(Arc<Collection>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = TauriResult<T>> + Send + 'static,
{
    moss_api::with_timeout(options, async move {
        let app_handle = app.handle();
        let (workspace, _ctx) = app
            .service::<WorkspaceService<R>>()
            .workspace_with_context(app_handle)
            .await
            .map_err_as_failed_precondition("No active workspace")?;

        let collection = workspace
            .service::<DynCollectionService>()
            .collection(&id)
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
        let app_handle = app.handle();
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
