use crate::commands::primitives::*;
use joinerror::ResultExt;
use moss_applib::TauriAppRuntime;
use moss_workspace::models::operations::*;
use sapic_ipc::contracts::main::{
    environment::{
        ActivateEnvironmentInput, ActivateEnvironmentOutput, BatchUpdateEnvironmentGroupInput,
        BatchUpdateEnvironmentInput, BatchUpdateEnvironmentOutput, CreateEnvironmentInput,
        CreateEnvironmentOutput, DeleteEnvironmentInput, DeleteEnvironmentOutput,
        StreamEnvironmentsEvent, StreamEnvironmentsOutput, StreamProjectEnvironmentsInput,
        StreamProjectEnvironmentsOutput, UpdateEnvironmentGroupInput, UpdateEnvironmentInput,
        UpdateEnvironmentOutput,
    },
    project::{
        ArchiveProjectInput, ArchiveProjectOutput, BatchUpdateProjectInput,
        BatchUpdateProjectOutput, DeleteProjectInput, DeleteProjectOutput, DescribeProjectInput,
        DescribeProjectOutput, ExportProjectInput, ExportProjectOutput, ImportProjectInput,
        ImportProjectOutput, UnarchiveProjectInput, UnarchiveProjectOutput, UpdateProjectInput,
        UpdateProjectOutput,
    },
};
use tauri::{Window, ipc::Channel as TauriChannel};
// Project

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn describe_project<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: DescribeProjectInput,
    options: Options,
) -> joinerror::Result<DescribeProjectOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.describe_project(&ctx, &input).await },
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
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.export_project(&ctx, &input).await },
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

// Environment
#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_environments<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    channel: TauriChannel<StreamEnvironmentsEvent>,
    options: Options,
) -> joinerror::Result<StreamEnvironmentsOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.stream_environments(&ctx, channel).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_project_environments<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: StreamProjectEnvironmentsInput,
    channel: TauriChannel<StreamEnvironmentsEvent>,
    options: Options,
) -> joinerror::Result<StreamProjectEnvironmentsOutput> {
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move {
            window
                .stream_project_environments(&ctx, input, channel)
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
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.activate_environment(&ctx, input).await },
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
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.create_environment(&ctx, input).await },
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
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.update_environment(&ctx, input).await },
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
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.batch_update_environment(&ctx, input).await },
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
    super::with_main_window_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, _, window| async move { window.delete_environment(&ctx, input).await },
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
    // super::with_workspace_timeout(
    //     ctx.inner(),
    //     app,
    //     window,
    //     options,
    //     |ctx, _, workspace| async move {
    //         workspace
    //             .update_environment_group::<TauriAppRuntime<R>>(&ctx, input)
    //             .await
    //     },
    // )
    // .await
    Ok(())
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
    // super::with_workspace_timeout(
    //     ctx.inner(),
    //     app,
    //     window,
    //     options,
    //     |ctx, _, workspace| async move {
    //         workspace
    //             .batch_update_environment_group::<TauriAppRuntime<R>>(&ctx, input)
    //             .await
    //     },
    // )
    // .await
    Ok(())
}
