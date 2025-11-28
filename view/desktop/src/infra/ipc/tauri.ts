import { InvokeArgs, invoke as invokeTauri } from "@tauri-apps/api/core";

//TODO divide this into chunks of commands
//window commands, plugin commands,
export type TauriIpcCommand =
  //
  // Plugins
  //
  | "plugin:shared-storage|get_item"
  | "plugin:shared-storage|put_item"
  | "plugin:shared-storage|remove_item"
  //
  // App
  //
  | "list_workspaces"
  | "describe_app"
  | "update_configuration"
  | "list_configuration_schemas"
  | "execute_command"
  | "get_translation_namespace"
  | "describe_color_theme"
  | "list_languages"
  | "list_color_themes"
  | "list_extensions"
  | "delete_workspace"
  | "update_workspace"
  | "update_profile"
  | "get_mistral_api_key"
  //
  // Main
  //
  | "main__cancel_request"
  | "main__update_workspace"
  | "main__close_workspace"
  | "main__open_workspace"
  | "main__create_workspace"
  //
  // Welcome
  //
  | "welcome__cancel_request"
  | "welcome__open_workspace"
  //
  // Workspace
  //
  | "describe_project"
  | "create_project"
  | "delete_project"
  | "stream_projects"
  | "update_project"
  | "archive_project"
  | "unarchive_project"
  | "batch_update_project"
  | "list_changes"
  | "import_project"
  | "export_project"
  | "stream_environments"
  | "create_environment"
  | "update_environment"
  | "batch_update_environment"
  | "delete_environment"
  | "update_environment_group"
  | "batch_update_environment_group"
  | "activate_environment"
  //
  // Project
  //
  | "create_project_resource"
  | "delete_project_resource"
  | "update_project_resource"
  | "stream_project_resources"
  | "describe_project_resource"
  | "batch_update_project_resource"
  | "batch_create_project_resource"
  | "execute_vcs_operation";

export type IpcResult<T, E> = { status: "ok"; data: T } | { status: "error"; error: E };

export const handleTauriIpcError = (cmd: TauriIpcCommand, error: unknown) => {
  console.error(`Error in IPC command "${cmd}":`, error);

  // TODO: dispatch to a global error handler or show user notifications
};

/**
 * @deprecated InvokeTauriServiceIpc should be used instead, specifically using services from the "view/desktop/src/lib/services" folder.
 */
export const invokeTauriIpc = async <T, E = unknown>(
  cmd: TauriIpcCommand,
  args?: InvokeArgs
): Promise<IpcResult<T, E>> => {
  try {
    const data = await invokeTauri<T>(cmd, args);
    return { status: "ok", data };
  } catch (err) {
    handleTauriIpcError(cmd, err);
    return { status: "error", error: err as E };
  }
};

interface InvokeTauriServiceIpcArgs<Input> {
  cmd: TauriIpcCommand;
  args?: {
    input?: Input extends undefined ? InvokeArgs : Input & InvokeArgs;
    options?: undefined;
  };
}

/**
 * @deprecated InvokeTauriServiceIpc is unneeded because services will provide a wrapper around the invoke function.
 */
export const invokeTauriServiceIpc = async <Input, Output, E = unknown>({
  cmd,
  args,
}: InvokeTauriServiceIpcArgs<Input>): Promise<IpcResult<Output, E>> => {
  try {
    const data = await invokeTauri<Output>(cmd, args);

    return {
      status: "ok",
      data,
    };
  } catch (err) {
    handleTauriIpcError(cmd, err);

    return {
      status: "error",
      error: err as E,
    };
  }
};
