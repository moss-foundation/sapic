mod app;
mod project;
mod window;
mod workspace;

pub use app::*;
pub use project::*;
pub use window::*;
pub use workspace::*;

use joinerror::OptionExt;
use moss_api::{TauriResult, constants::DEFAULT_OPERATION_TIMEOUT};
use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    context::{AnyAsyncContext, AnyContext},
    errors::{FailedPrecondition, NotFound, Unavailable},
};
use moss_project::Project;
use moss_workspace::models::primitives::ProjectId;
use primitives::Options;
use sapic_window::{ActiveWorkspace, window::Window};
use std::{sync::Arc, time::Duration};
use tauri::{Manager, State, Window as TauriWindow};

pub mod primitives {
    use std::sync::Arc;

    use tauri::State;

    pub(super) type Options = Option<moss_api::models::types::Options>;
    pub(super) type AsyncContext<'a> = State<'a, moss_applib::context::AsyncContext>;
    pub(super) type App<'a, R> = State<'a, Arc<sapic_app::App<moss_applib::TauriAppRuntime<R>>>>;
}

pub(super) async fn with_window_timeout<'a, R, T, F, Fut>(
    ctx: &R::AsyncContext,
    app: State<'_, Arc<sapic_app::App<R>>>,
    window: TauriWindow<R::EventLoop>,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: AppRuntime,
    F: FnOnce(R::AsyncContext, AppDelegate<R>, Arc<Window<R>>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = joinerror::Result<T>> + Send + 'static,
{
    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);
    let mut ctx = R::AsyncContext::new_with_timeout(ctx.clone(), timeout);
    let request_id = options.and_then(|opts| opts.request_id);

    let window = app
        .window(window.label())
        .await
        .ok_or_join_err_with::<Unavailable>(|| {
            format!("window '{}' is unavailable", window.label())
        })?;

    if let Some(request_id) = &request_id {
        ctx.with_value("request_id", request_id.clone());
        window
            .track_cancellation(request_id, ctx.get_canceller())
            .await;
    }

    let result = f(
        ctx.freeze(),
        app.handle().state::<AppDelegate<R>>().inner().clone(),
        window.clone(),
    )
    .await;

    if let Some(request_id) = &request_id {
        window.release_cancellation(request_id).await;
    }

    result.map_err(|e| e.into())
}

pub(super) async fn with_project_timeout<R, T, F, Fut>(
    ctx: &R::AsyncContext,
    app: State<'_, Arc<sapic_app::App<R>>>,
    window: TauriWindow<R::EventLoop>,
    id: ProjectId,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: AppRuntime,
    F: FnOnce(R::AsyncContext, AppDelegate<R>, Arc<Project<R>>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = joinerror::Result<T>> + Send + 'static,
{
    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);

    let mut ctx = R::AsyncContext::new_with_timeout(ctx.clone(), timeout);
    let window = app
        .window(window.label())
        .await
        .ok_or_join_err_with::<Unavailable>(|| {
            format!("window '{}' is unavailable", window.label())
        })?;

    let workspace = window
        .workspace()
        .await
        .ok_or_join_err::<FailedPrecondition>("no active workspace")?;

    let project = workspace
        .project(&id)
        .await
        .ok_or_join_err::<NotFound>("project is not found")?;

    let request_id = options.and_then(|opts| opts.request_id);

    if let Some(request_id) = &request_id {
        ctx.with_value("request_id", request_id.clone());
        window
            .track_cancellation(&request_id, ctx.get_canceller())
            .await;
    }

    let result = f(
        ctx.freeze(),
        app.handle().state::<AppDelegate<R>>().inner().clone(),
        project,
    )
    .await;

    if let Some(request_id) = &request_id {
        window.release_cancellation(request_id).await;
    }
    result.map_err(|e| e.into())
}

pub(super) async fn with_workspace_timeout<R, T, F, Fut>(
    ctx: &R::AsyncContext,
    app: State<'_, Arc<sapic_app::App<R>>>,
    window: TauriWindow<R::EventLoop>,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: AppRuntime,
    F: FnOnce(R::AsyncContext, AppDelegate<R>, Arc<ActiveWorkspace<R>>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = joinerror::Result<T>> + Send + 'static,
{
    let window = app
        .window(window.label())
        .await
        .ok_or_join_err_with::<Unavailable>(|| {
            format!("window '{}' is unavailable", window.label())
        })?;

    let workspace = window
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
        window
            .track_cancellation(request_id, ctx.get_canceller())
            .await;
    }

    let result = f(
        ctx.freeze(),
        app.handle().state::<AppDelegate<R>>().inner().clone(),
        workspace,
    )
    .await;

    if let Some(request_id) = &request_id {
        window.release_cancellation(request_id).await;
    }

    result.map_err(|e| e.into())
}
