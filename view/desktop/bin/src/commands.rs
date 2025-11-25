mod app;
mod main;
mod project;
mod welcome;
mod window;
mod workspace;

pub use app::*;
pub use main::*;
pub use project::*;
pub use welcome::*;
pub use window::*;
pub use workspace::*;

use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    errors::{FailedPrecondition, NotFound, Unavailable},
};
use moss_project::{Project, models::primitives::ProjectId};
use primitives::Options;
use sapic_core::context::{AnyAsyncContext, ArcContext, ContextBuilder};
use sapic_ipc::{TauriResult, constants::DEFAULT_OPERATION_TIMEOUT};
use sapic_main::MainWindow;
use sapic_welcome::WelcomeWindow;
use sapic_window2::AppWindowApi;
use std::{sync::Arc, time::Duration};
use tauri::{Manager, State, Window as TauriWindow};

pub mod primitives {
    use std::sync::Arc;

    use tauri::State;

    pub(super) type Options = Option<sapic_ipc::contracts::Options>;
    pub(super) type AsyncContext<'a> = State<'a, sapic_core::context::ArcContext>;
    pub(super) type App<'a, R> = State<'a, Arc<sapic_app::App<moss_applib::TauriAppRuntime<R>>>>;
}

pub(super) async fn with_app_timeout<'a, R, T, F, Fut>(
    ctx: &R::AsyncContext,
    app: State<'_, Arc<sapic_app::App<R>>>,
    window: TauriWindow<R::EventLoop>,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: AppRuntime<AsyncContext = ArcContext>,
    F: FnOnce(R::AsyncContext, Arc<sapic_app::App<R>>, AppDelegate<R>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = joinerror::Result<T>> + Send + 'static,
{
    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);

    let request_id = options.and_then(|opts| opts.request_id);
    let mut builder = ContextBuilder::new()
        .with_parent(ctx.clone())
        .with_timeout(timeout);

    if let Some(ref request_id) = request_id {
        builder = builder.with_value("request_id", request_id.clone());
    }

    let ctx = builder.freeze();

    let window = app
        .window(window.label())
        .await
        .ok_or_join_err_with::<Unavailable>(|| {
            format!("window '{}' is unavailable", window.label())
        })?;

    if let Some(request_id) = &request_id {
        window
            .track_cancellation(request_id, ctx.get_canceller())
            .await;
    }

    let delegate = app.handle().state::<AppDelegate<R>>().inner().clone();
    let result = f(ctx, app.inner().clone(), delegate).await;

    if let Some(request_id) = &request_id {
        window.release_cancellation(request_id).await;
    }

    result.map_err(|e| e.into())
}

pub(super) async fn with_welcome_window_timeout<'a, R, T, F, Fut>(
    ctx: &R::AsyncContext,
    app: State<'_, Arc<sapic_app::App<R>>>,
    _window: TauriWindow<R::EventLoop>,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: AppRuntime<AsyncContext = ArcContext>,
    F: FnOnce(R::AsyncContext, Arc<sapic_app::App<R>>, AppDelegate<R>, WelcomeWindow<R>) -> Fut
        + Send
        + 'static,
    Fut: std::future::Future<Output = joinerror::Result<T>> + Send + 'static,
{
    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);

    let request_id = options.and_then(|opts| opts.request_id);
    let mut builder = ContextBuilder::new()
        .with_parent(ctx.clone())
        .with_timeout(timeout);

    if let Some(ref request_id) = request_id {
        builder = builder.with_value("request_id", request_id.clone());
    }

    let ctx = builder.freeze();

    let window = app
        .welcome_window()
        .await
        .ok_or_join_err_with::<Unavailable>(|| format!("welcome window is unavailable"))?;

    if let Some(request_id) = &request_id {
        window
            .track_cancellation(request_id, ctx.get_canceller())
            .await;
    }

    let delegate = app.handle().state::<AppDelegate<R>>().inner().clone();
    let result = f(ctx, app.inner().clone(), delegate, window.clone()).await;

    if let Some(request_id) = &request_id {
        window.release_cancellation(request_id).await;
    }

    result.map_err(|e| e.into())
}

pub(super) async fn with_main_window_timeout<'a, R, T, F, Fut>(
    ctx: &R::AsyncContext,
    app: State<'_, Arc<sapic_app::App<R>>>,
    window: TauriWindow<R::EventLoop>,
    options: Options,
    f: F,
) -> TauriResult<T>
where
    R: AppRuntime<AsyncContext = ArcContext>,
    F: FnOnce(R::AsyncContext, Arc<sapic_app::App<R>>, AppDelegate<R>, MainWindow<R>) -> Fut
        + Send
        + 'static,
    Fut: std::future::Future<Output = joinerror::Result<T>> + Send + 'static,
{
    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);
    let request_id = options.and_then(|opts| opts.request_id);
    let mut builder = ContextBuilder::new()
        .with_parent(ctx.clone())
        .with_timeout(timeout);

    if let Some(ref request_id) = request_id {
        builder = builder.with_value("request_id", request_id.clone());
    }

    let ctx = builder.freeze();

    let window = app
        .main_window(window.label())
        .await
        .ok_or_join_err_with::<Unavailable>(|| {
            format!("window '{}' is unavailable", window.label())
        })?;

    if let Some(request_id) = &request_id {
        window
            .track_cancellation(request_id, ctx.get_canceller())
            .await;
    }

    let delegate = app.handle().state::<AppDelegate<R>>().inner().clone();
    let result = f(ctx, app.inner().clone(), delegate, window.clone()).await;

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
    R: AppRuntime<AsyncContext = ArcContext>,
    F: FnOnce(R::AsyncContext, AppDelegate<R>, Arc<Project<R>>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = joinerror::Result<T>> + Send + 'static,
{
    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);

    let request_id = options.and_then(|opts| opts.request_id);
    let mut builder = ContextBuilder::new()
        .with_parent(ctx.clone())
        .with_timeout(timeout);

    if let Some(ref request_id) = request_id {
        builder = builder.with_value("request_id", request_id.clone());
    }

    let ctx = builder.freeze();

    let window = app
        .main_window(window.label())
        .await
        .ok_or_join_err_with::<Unavailable>(|| {
            format!("window '{}' is unavailable", window.label())
        })?;

    let workspace = window
        .inner()
        .workspace()
        .await
        .ok_or_join_err::<FailedPrecondition>("no active workspace")?;

    let project = workspace
        .project(&id)
        .await
        .ok_or_join_err::<NotFound>("project is not found")?;

    if let Some(request_id) = &request_id {
        window
            .track_cancellation(&request_id, ctx.get_canceller())
            .await;
    }

    let result = f(
        ctx,
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
    R: AppRuntime<AsyncContext = ArcContext>,
    F: FnOnce(R::AsyncContext, AppDelegate<R>, Arc<moss_workspace::Workspace<R>>) -> Fut
        + Send
        + 'static,
    Fut: std::future::Future<Output = joinerror::Result<T>> + Send + 'static,
{
    let window = app
        .main_window(window.label())
        .await
        .ok_or_join_err_with::<Unavailable>(|| {
            format!("window '{}' is unavailable", window.label())
        })?;

    let workspace = window
        .inner()
        .workspace()
        .await
        .ok_or_join_err::<FailedPrecondition>("no active workspace")?;

    let timeout = options
        .as_ref()
        .and_then(|opts| opts.timeout.map(Duration::from_secs))
        .unwrap_or(DEFAULT_OPERATION_TIMEOUT);

    let request_id = options.and_then(|opts| opts.request_id);
    let mut builder = ContextBuilder::new()
        .with_parent(ctx.clone())
        .with_timeout(timeout);

    if let Some(ref request_id) = request_id {
        builder = builder.with_value("request_id", request_id.clone());
    }

    let ctx = builder.freeze();

    if let Some(request_id) = &request_id {
        window
            .track_cancellation(request_id, ctx.get_canceller())
            .await;
    }

    let result = f(
        ctx,
        app.handle().state::<AppDelegate<R>>().inner().clone(),
        workspace,
    )
    .await;

    if let Some(request_id) = &request_id {
        window.release_cancellation(request_id).await;
    }

    result.map_err(|e| e.into())
}
