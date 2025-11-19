use moss_workspace::{
    api::BatchUpdateProjectOp,
    models::{events::*, operations::*},
};
use sapic_ipc::TauriResult;
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
) -> TauriResult<StreamEnvironmentsOutput> {
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
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_projects<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    channel: TauriChannel<StreamProjectsEvent>,
    options: Options,
) -> TauriResult<StreamProjectsOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.stream_projects(&ctx, channel).await },
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
) -> TauriResult<DescribeProjectOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.describe_project(&ctx, &input).await },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_project<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    input: CreateProjectInput,
    options: Options,
) -> TauriResult<CreateProjectOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app_delegate, workspace| async move {
            workspace.create_project(&ctx, &app_delegate, &input).await
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
) -> TauriResult<ImportProjectOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, app_delegate, workspace| async move {
            workspace.import_project(&ctx, &app_delegate, &input).await
        },
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
) -> TauriResult<ExportProjectOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.export_project(&ctx, &input).await },
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
) -> TauriResult<DeleteProjectOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.delete_project(&ctx, &input).await },
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
) -> TauriResult<UpdateProjectOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.update_project(&ctx, input).await },
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
) -> TauriResult<ArchiveProjectOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.archive_project(&ctx, input).await },
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
) -> TauriResult<UnarchiveProjectOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.unarchive_project(&ctx, input).await },
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
) -> TauriResult<BatchUpdateProjectOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.batch_update_project(&ctx, input).await },
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
) -> TauriResult<ListChangesOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.list_changes(&ctx).await },
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
) -> TauriResult<ActivateEnvironmentOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.activate_environment(&ctx, input).await },
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
) -> TauriResult<CreateEnvironmentOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, appp_delegate, workspace| async move {
            workspace
                .create_environment(&ctx, appp_delegate, input)
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
) -> TauriResult<UpdateEnvironmentOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.update_environment(&ctx, input).await },
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
) -> TauriResult<BatchUpdateEnvironmentOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.batch_update_environment(&ctx, input).await },
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
) -> TauriResult<DeleteEnvironmentOutput> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.delete_environment(&ctx, input).await },
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
) -> TauriResult<()> {
    super::with_workspace_timeout(
        ctx.inner(),
        app,
        window,
        options,
        |ctx, _, workspace| async move { workspace.update_environment_group(&ctx, input).await },
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
) -> TauriResult<()> {
    super::with_workspace_timeout(ctx.inner(), app,
        window,
        options,
        |ctx, _, workspace| async move {
        workspace.batch_update_environment_group(&ctx, input).await
    })
    .await
}
