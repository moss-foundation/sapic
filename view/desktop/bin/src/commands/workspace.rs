use joinerror::ResultExt;
use moss_applib::TauriAppRuntime;
use moss_workspace::models::{events::*, operations::*};
use sapic_ipc::contracts::main::project::{
    ArchiveProjectInput, ArchiveProjectOutput, BatchUpdateProjectInput, BatchUpdateProjectOutput,
    DeleteProjectInput, DeleteProjectOutput, ImportProjectInput, ImportProjectOutput,
    UnarchiveProjectInput, UnarchiveProjectOutput, UpdateProjectInput, UpdateProjectOutput,
};
use tauri::{Window, ipc::Channel as TauriChannel};

use crate::commands::primitives::*;

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_environments<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    channel: TauriChannel<StreamEnvironmentsEvent>,
    options: Options,
) -> joinerror::Result<StreamEnvironmentsOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app_delegate, workspace| async move {
            workspace
                .stream_environments(&ctx, app_delegate, channel)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn describe_project<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: DescribeProjectInput,
    options: Options,
) -> joinerror::Result<DescribeProjectOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move {
            workspace
                .describe_project::<TauriAppRuntime<R>>(&ctx, &input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn import_project<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: ImportProjectInput,
    options: Options,
) -> joinerror::Result<ImportProjectOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.import_project(&ctx, &input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn export_project<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: ExportProjectInput,
    options: Options,
) -> joinerror::Result<ExportProjectOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move {
            workspace
                .export_project::<TauriAppRuntime<R>>(&ctx, &input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn delete_project<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: DeleteProjectInput,
    options: Options,
) -> joinerror::Result<DeleteProjectOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move {
            window
                .delete_project(&ctx, &input)
                .await
                .join_err::<()>("failed to delete project")
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_project<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: UpdateProjectInput,
    options: Options,
) -> joinerror::Result<UpdateProjectOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move {
            window
                .update_project(&ctx, &input)
                .await
                .join_err::<()>("failed to update project")
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn archive_project<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: ArchiveProjectInput,
    options: Options,
) -> joinerror::Result<ArchiveProjectOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.archive_project(&ctx, input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn unarchive_project<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: UnarchiveProjectInput,
    options: Options,
) -> joinerror::Result<UnarchiveProjectOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.unarchive_project(&ctx, input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn batch_update_project<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: BatchUpdateProjectInput,
    options: Options,
) -> joinerror::Result<BatchUpdateProjectOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move {
            window
                .batch_update_project(&ctx, input)
                .await
                .join_err::<()>("failed to batch update project")
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn list_changes<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    options: Options,
) -> joinerror::Result<ListChangesOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app_delegate, workspace| async move {
            workspace
                .list_changes::<TauriAppRuntime<R>>(&ctx, &app_delegate)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn activate_environment<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: ActivateEnvironmentInput,
    options: Options,
) -> joinerror::Result<ActivateEnvironmentOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move {
            workspace
                .activate_environment::<TauriAppRuntime<R>>(&ctx, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_environment<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: CreateEnvironmentInput,
    options: Options,
) -> joinerror::Result<CreateEnvironmentOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, appp_delegate, workspace| async move {
            workspace
                .create_environment::<TauriAppRuntime<R>>(&ctx, appp_delegate, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_environment<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: UpdateEnvironmentInput,
    options: Options,
) -> joinerror::Result<UpdateEnvironmentOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move {
            workspace
                .update_environment::<TauriAppRuntime<R>>(&ctx, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn batch_update_environment<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: BatchUpdateEnvironmentInput,
    options: Options,
) -> joinerror::Result<BatchUpdateEnvironmentOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move {
            workspace
                .batch_update_environment::<TauriAppRuntime<R>>(&ctx, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn delete_environment<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: DeleteEnvironmentInput,
    options: Options,
) -> joinerror::Result<DeleteEnvironmentOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move {
            workspace
                .delete_environment::<TauriAppRuntime<R>>(&ctx, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn update_environment_group<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: UpdateEnvironmentGroupInput,
    options: Options,
) -> joinerror::Result<()> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move {
            workspace
                .update_environment_group::<TauriAppRuntime<R>>(&ctx, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app), fields(window = window.label()))]
pub async fn batch_update_environment_group<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: BatchUpdateEnvironmentGroupInput,
    options: Options,
) -> joinerror::Result<()> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move {
            workspace
                .batch_update_environment_group::<TauriAppRuntime<R>>(&ctx, input)
                .await
        },
    )
    .await
}
