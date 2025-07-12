mod app;
mod collection;
mod workspace;

pub use app::*;
pub use collection::*;
use moss_applib::ctx::{self, AsyncContext, Context, MutableContext};
pub use workspace::*;

use anyhow::Context as _;
use moss_api::{TauriError, TauriResult, constants::DEFAULT_OPERATION_TIMEOUT};
use moss_app::{
    app::App,
    services::workspace_service::{ActiveWorkspace, WorkspaceService},
};
use moss_collection::Collection;
use moss_common::api::OperationOptionExt;
use moss_workspace::{models::primitives::CollectionId, services::DynCollectionService};
use std::{sync::Arc, time::Duration};
use tauri::{Runtime as TauriRuntime, State};

pub(super) type Options = Option<moss_api::models::types::Options>;

pub(super) async fn with_collection_timeout<R, T, F, Fut>(
    ctx: AsyncContext,
    app: State<'_, App<R>>,
    id: CollectionId,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: TauriRuntime,
    F: FnOnce(AsyncContext, Arc<Collection>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = TauriResult<T>> + Send + 'static,
{
    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);

    let mut ctx = MutableContext::from(&ctx);
    ctx.with_timeout(timeout);

    if let Some(request_id) = options.and_then(|opts| opts.request_id) {
        ctx.with_value("request_id", request_id);
    }

    let workspace = app
        .service::<WorkspaceService<R>>()
        .workspace()
        .await
        .map_err_as_failed_precondition("No active workspace")?;

    let collection = workspace
        .service::<DynCollectionService>()
        .collection(&id)
        .await
        .context("Collection not found")?;

    f(ctx.freeze(), collection).await
}

pub(super) async fn with_workspace_timeout<R, T, F, Fut>(
    app: State<'_, App<R>>,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: TauriRuntime,
    F: FnOnce(AsyncContext, Arc<ActiveWorkspace<R>>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = TauriResult<T>> + Send + 'static,
{
    let timeout = options
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);

    let mut ctx = MutableContext::background();
    ctx.with_timeout(timeout);

    let workspace = app
        .service::<WorkspaceService<R>>()
        .workspace()
        .await
        .map_err_as_failed_precondition("No active workspace")?;

    f(ctx.freeze(), workspace).await
}
