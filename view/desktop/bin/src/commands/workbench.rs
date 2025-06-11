use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use moss_app::{
    context::{AppContext, Context, Global},
    manager::AppManager,
};
use moss_tauri::{TauriError, TauriResult};
use moss_workbench::{models::operations::*, workbench::Workbench};
use tauri::{AppHandle, Manager, Runtime as TauriRuntime, State, Window};

#[tauri::command(async)]
#[instrument(level = "trace", skip(workbench, ctx), fields(window = window.label()))]
pub async fn create_workspace<R: TauriRuntime>(
    ctx: State<'_, AppContext<R>>,
    workbench: State<'_, Arc<Workbench<R>>>,
    window: Window<R>,
    input: CreateWorkspaceInput,
) -> TauriResult<CreateWorkspaceOutput> {
    // let app_handle = app_manager.app_handle();
    // let workbench = app_manager
    //     .services()
    //     .get_by_type::<Workbench<R>>(&app_handle)
    //     .await?;

    // let workspace_output = workbench
    //     .create_workspace(&*ctx, &input)
    //     .await
    //     .map_err(|err| TauriError(format!("Failed to create workspace: {}", err)))?; // TODO: improve error handling

    let workbench_owned = (*workbench).clone();

    let task = ctx.spawn::<CreateWorkspaceOutput, TauriError, _, _>(
        move |cx| async move {
            let workspace_output = workbench_owned
                .create_workspace(&cx, &input)
                .await
                .map_err(|err| TauriError(format!("Failed to create workspace: {}", err)))?;

            Ok(workspace_output)
        },
        None,
    );

    match task.await {
        moss_app::context::TaskResult::Ok(result) => Ok(result),
        moss_app::context::TaskResult::Err(err) => Err(err),
        moss_app::context::TaskResult::Timeout => Err(TauriError("Task timed out".to_string())),
        moss_app::context::TaskResult::Cancelled => {
            Err(TauriError("Task was cancelled".to_string()))
        }
    }

    // Ok(workspace_output)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager, ctx), fields(window = window.label()))]
pub async fn list_workspaces<R: TauriRuntime>(
    ctx: State<'_, AppContext<R>>,
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
) -> TauriResult<ListWorkspacesOutput> {
    let app_handle = app_manager.app_handle();
    let workbench = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    workbench
        .list_workspaces(&ctx)
        .await
        .map_err(|err| TauriError(format!("Failed to list workspaces: {}", err))) // TODO: improve error handling
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager, ctx), fields(window = window.label()))]
pub async fn delete_workspace<R: TauriRuntime>(
    ctx: State<'_, AppContext<R>>,
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: DeleteWorkspaceInput,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let workbench = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    workbench
        .delete_workspace(&*ctx, &input)
        .await
        .map_err(|err| TauriError(format!("Failed to delete workspace: {}", err))) // TODO: improve error handling
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager, ctx), fields(window = window.label()))]
pub async fn open_workspace<R: TauriRuntime>(
    ctx: State<'_, AppContext<R>>,
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: OpenWorkspaceInput,
) -> TauriResult<OpenWorkspaceOutput> {
    let app_handle = app_manager.app_handle();
    let workbench = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    workbench
        .open_workspace(&ctx, &input)
        .await
        .map_err(|err| TauriError(format!("Failed to open workspace: {}", err))) // TODO: improve error handling
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager, ctx), fields(window = window.label()))]
pub async fn update_workspace<R: TauriRuntime>(
    ctx: State<'_, AppContext<R>>,
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
    input: UpdateWorkspaceInput,
) -> TauriResult<()> {
    let app_handle = app_manager.app_handle();
    let workbench = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    workbench
        .update_workspace(&ctx, &input)
        .await
        .map_err(|err| TauriError(format!("Failed to update workspace: {}", err))) // TODO: improve error handling
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager, _ctx), fields(window = window.label()))]
pub async fn describe_workbench_state<R: TauriRuntime>(
    _ctx: State<'_, AppContext<R>>,
    app_manager: State<'_, AppManager<R>>,
    window: Window<R>,
) -> TauriResult<DescribeWorkbenchStateOutput> {
    let app_handle = app_manager.app_handle();
    let workbench = app_manager
        .services()
        .get_by_type::<Workbench<R>>(&app_handle)
        .await?;

    workbench
        .describe_state()
        .await
        .map_err(|err| TauriError(format!("Failed to describe workbench state: {}", err))) // TODO: improve error handling
}
