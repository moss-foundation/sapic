use moss_applib::TauriAppRuntime;
use moss_project::models::{events::*, operations::*};
use sapic_base::{project::types::primitives::ProjectId, resource::types::primitives::ResourceId};
use tauri::{Window, ipc::Channel as TauriChannel};

use crate::commands::primitives::*;

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn create_project_resource<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    input: CreateResourceInput,
    options: Options,
) -> joinerror::Result<CreateResourceOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        window,
        project_id,
        options,
        |ctx, _, project| async move {
            project
                .create_resource::<TauriAppRuntime<R>>(&ctx, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn delete_project_resource<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    input: DeleteResourceInput,
    options: Options,
) -> joinerror::Result<DeleteResourceOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        window,
        project_id,
        options,
        |ctx, _, project| async move {
            project
                .delete_resource::<TauriAppRuntime<R>>(&ctx, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn update_project_resource<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    input: UpdateResourceInput,
    options: Options,
) -> joinerror::Result<UpdateResourceOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        window,
        project_id,
        options,
        |ctx, app_delegate, project| async move {
            project.update_resource(&ctx, &app_delegate, input).await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn batch_create_project_resource<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    input: BatchCreateResourceInput,
    options: Options,
) -> joinerror::Result<BatchCreateResourceOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        window,
        project_id,
        options,
        |ctx, _, project| async move {
            project
                .batch_create_resource::<TauriAppRuntime<R>>(&ctx, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn batch_update_project_resource<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    channel: TauriChannel<BatchUpdateResourceEvent>,
    project_id: ProjectId,
    input: BatchUpdateResourceInput,
    options: Options,
) -> joinerror::Result<BatchUpdateResourceOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        window,
        project_id,
        options,
        |ctx, app_delegate, project| async move {
            project
                .batch_update_resource(&ctx, &app_delegate, input, channel)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label(), channel = channel.id()))]
pub async fn stream_project_resources<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    input: StreamResourcesInput,
    channel: TauriChannel<StreamResourcesEvent>,
    options: Options,
) -> joinerror::Result<StreamResourcesOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        window,
        project_id,
        options,
        |ctx, app_delegate, project| async move {
            project
                .stream_resources(&ctx, &app_delegate, channel, input)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn describe_project_resource<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    resource_id: ResourceId,
    options: Options,
) -> joinerror::Result<DescribeResourceOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        window,
        project_id,
        options,
        |ctx, app_delegate, project| async move {
            project
                .describe_resource(&ctx, &app_delegate, resource_id)
                .await
        },
    )
    .await
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(ctx, app), fields(window = window.label()))]
pub async fn execute_vcs_operation<'a, R: tauri::Runtime>(
    ctx: AsyncContext<'a>,
    app: App<'a, R>,
    window: Window<R>,
    project_id: ProjectId,
    input: ExecuteVcsOperationInput,
    options: Options,
) -> joinerror::Result<ExecuteVcsOperationOutput> {
    super::with_project_timeout(
        ctx.inner(),
        app,
        window,
        project_id,
        options,
        |ctx, app_delegate, collection| async move {
            collection
                .execute_vcs_operation(&ctx, &app_delegate, input)
                .await
        },
    )
    .await
}
