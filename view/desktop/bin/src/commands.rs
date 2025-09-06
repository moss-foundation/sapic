mod app;
mod collection;
mod env;
mod workspace;

pub use app::*;
pub use collection::*;
pub use env::*;
pub use workspace::*;

use joinerror::OptionExt;
use moss_api::{TauriResult, constants::DEFAULT_OPERATION_TIMEOUT};
use moss_app::{ActiveWorkspace, app::App};
use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    context::{AnyAsyncContext, AnyContext},
    errors::{FailedPrecondition, NotFound},
};
use moss_collection::Collection;
use moss_workspace::models::primitives::CollectionId;
use primitives::Options;
use std::{sync::Arc, time::Duration};
use tauri::{Manager, State};

pub mod primitives {
    use std::sync::Arc;

    use tauri::State;

    pub(super) type Options = Option<moss_api::models::types::Options>;
    pub(super) type AsyncContext<'a> = State<'a, moss_applib::context::AsyncContext>;
    pub(super) type App<'a, R> = State<'a, Arc<moss_app::App<moss_applib::TauriAppRuntime<R>>>>;
}

pub(super) async fn with_app_timeout<'a, R, T, F, Fut>(
    ctx: &R::AsyncContext,
    app: State<'_, Arc<App<R>>>,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: AppRuntime,
    F: FnOnce(R::AsyncContext, AppDelegate<R>, Arc<App<R>>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = joinerror::Result<T>> + Send + 'static,
{
    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);
    let mut ctx = R::AsyncContext::new_with_timeout(ctx.clone(), timeout);
    let request_id = options.and_then(|opts| opts.request_id);

    if let Some(request_id) = &request_id {
        ctx.with_value("request_id", request_id.clone());
        app.track_cancellation(request_id, ctx.get_canceller())
            .await;
    }

    let result = f(
        ctx.freeze(),
        app.handle().state::<AppDelegate<R>>().inner().clone(),
        app.inner().clone(),
    )
    .await;

    if let Some(request_id) = &request_id {
        app.release_cancellation(request_id).await;
    }

    result.map_err(|e| e.into())
}

pub(super) async fn with_collection_timeout<R, T, F, Fut>(
    ctx: &R::AsyncContext,
    app: State<'_, Arc<App<R>>>,
    id: CollectionId,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: AppRuntime,
    F: FnOnce(R::AsyncContext, AppDelegate<R>, Arc<Collection<R>>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = joinerror::Result<T>> + Send + 'static,
{
    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);

    let mut ctx = R::AsyncContext::new_with_timeout(ctx.clone(), timeout);

    let workspace = app
        .workspace()
        .await
        .ok_or_join_err::<FailedPrecondition>("no active workspace")?;

    let collection = workspace
        .collection(&id)
        .await
        .ok_or_join_err::<NotFound>("Collection is not found")?;

    let request_id = options.and_then(|opts| opts.request_id);

    if let Some(request_id) = &request_id {
        ctx.with_value("request_id", request_id.clone());
        app.track_cancellation(&request_id, ctx.get_canceller())
            .await;
    }

    let result = f(
        ctx.freeze(),
        app.handle().state::<AppDelegate<R>>().inner().clone(),
        collection,
    )
    .await;

    if let Some(request_id) = &request_id {
        app.release_cancellation(request_id).await;
    }
    result.map_err(|e| e.into())
}

pub(super) async fn with_workspace_timeout<R, T, F, Fut>(
    ctx: &R::AsyncContext,
    app: State<'_, Arc<App<R>>>,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: AppRuntime,
    F: FnOnce(R::AsyncContext, AppDelegate<R>, Arc<ActiveWorkspace<R>>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = joinerror::Result<T>> + Send + 'static,
{
    let workspace = app
        .workspace()
        .await
        .ok_or_join_err::<FailedPrecondition>("no active workspace")?;

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

    let result = f(
        ctx.freeze(),
        app.handle().state::<AppDelegate<R>>().inner().clone(),
        workspace,
    )
    .await;

    if let Some(request_id) = &request_id {
        app.release_cancellation(request_id).await;
    }

    result.map_err(|e| e.into())
}
