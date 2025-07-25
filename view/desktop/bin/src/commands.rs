mod app;
mod collection;
mod workspace;

pub use app::*;
pub use collection::*;
pub use workspace::*;

use moss_api::{TauriResult, constants::DEFAULT_OPERATION_TIMEOUT};
use moss_app::{
    app::App,
    services::workspace_service::{ActiveWorkspace, WorkspaceService},
};
use moss_applib::{
    AppRuntime,
    context::{AnyAsyncContext, AnyContext},
};
use moss_collection::Collection;
use moss_common::api::OperationOptionExt;
use moss_workspace::{models::primitives::CollectionId, services::DynCollectionService};
use primitives::Options;
use std::{sync::Arc, time::Duration};
use tauri::State;

pub mod primitives {
    use tauri::State;

    pub(super) type Options = Option<moss_api::models::types::Options>;
    pub(super) type AsyncContext<'a> = State<'a, moss_applib::context::AsyncContext>;
    pub(super) type App<'a, R> = State<'a, moss_app::App<moss_applib::TauriAppRuntime<R>>>;
}

pub(super) async fn with_collection_timeout<R, T, F, Fut>(
    ctx: &R::AsyncContext,
    app: State<'_, App<R>>,
    id: CollectionId,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: AppRuntime,
    F: FnOnce(R::AsyncContext, Arc<Collection<R>>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = TauriResult<T>> + Send + 'static,
{
    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);

    let mut ctx = R::AsyncContext::new_with_timeout(ctx.clone(), timeout);

    let workspace = app
        .service::<WorkspaceService<R>>()
        .workspace()
        .await
        .map_err_as_failed_precondition("No active workspace")?;

    let collection = workspace
        .service::<DynCollectionService<R>>()
        .collection(&id)
        .await?;

    let request_id = options.and_then(|opts| opts.request_id);

    if let Some(request_id) = &request_id {
        ctx.with_value("request_id", request_id.clone());
        app.track_cancellation(&request_id, ctx.get_canceller())
            .await;
    }

    let result = f(ctx.freeze(), collection).await;

    if let Some(request_id) = &request_id {
        app.release_cancellation(request_id).await;
    }
    result
}

pub(super) async fn with_workspace_timeout<R, T, F, Fut>(
    ctx: &R::AsyncContext,
    app: State<'_, App<R>>,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: AppRuntime,
    F: FnOnce(R::AsyncContext, Arc<ActiveWorkspace<R>>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = TauriResult<T>> + Send + 'static,
{
    let workspace = app
        .service::<WorkspaceService<R>>()
        .workspace()
        .await
        .map_err_as_failed_precondition("No active workspace")?;

    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);

    let request_id = options.and_then(|opts| opts.request_id);

    let mut ctx = R::AsyncContext::new_with_timeout(ctx.clone(), timeout);
    if let Some(request_id) = &request_id {
        ctx.with_value("request_id", request_id.clone());
        app.track_cancellation(request_id, ctx.get_canceller())
            .await;
    }

    let result = f(ctx.freeze(), workspace).await;

    if let Some(request_id) = &request_id {
        app.release_cancellation(request_id).await;
    }
    result
}
