import { InvokeArgs, invoke as invokeTauri } from "@tauri-apps/api/core";
import type { EventCallback, EventName } from "@tauri-apps/api/event";
import { listen as listenTauri } from "@tauri-apps/api/event";

export type TauriIpcCommand =
  //
  // App
  //
  | "describe_app"
  | "update_configuration"
  | "list_configuration_schemas"
  | "set_color_theme" // DEPRECATED
  | "set_locale" // DEPRECATED
  | "execute_command"
  | "get_locale"
  | "get_translation_namespace"
  | "describe_color_theme"
  | "list_locales"
  | "list_color_themes"
  | "create_workspace"
  | "open_workspace"
  | "list_workspaces"
  | "delete_workspace"
  | "update_workspace"
  | "close_workspace"
  | "update_profile"
  | "get_mistral_api_key"
  //
  // Workspace
  //
  | "update_workspace_state" // DEPRECATED
  | "update_layout"
  | "describe_workspace"
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
  | "create_project_entry"
  | "delete_project_entry"
  | "update_project_entry"
  | "stream_project_entries"
  | "describe_project_entry"
  | "batch_update_project_entry"
  | "batch_create_project_entry"
  | "execute_vcs_operation";

export type IpcResult<T, E> = { status: "ok"; data: T } | { status: "error"; error: E };

export const handleTauriIpcError = (cmd: TauriIpcCommand, error: unknown) => {
  console.error(`Error in IPC command "${cmd}":`, error);

  // TODO: dispatch to a global error handler or show user notifications
};

/**
 * @deprecated InvokeTauriServiceIpc should be used instead, specifically from services folder.
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

export const invokeTauriServiceIpc = async <Input, Output, E = unknown>({
  cmd,
  args,
}: InvokeTauriServiceIpcArgs<Input>): Promise<IpcResult<Output, E>> => {
  try {
    const data = await invokeTauri<Output>(cmd, args);
    return { status: "ok", data };
  } catch (err) {
    handleTauriIpcError(cmd, err);
    return { status: "error", error: err as E };
  }
};

export const listenTauriIpc = <T>(event: EventName, handle: EventCallback<T>) => {
  const unlisten = listenTauri(event, handle);
  return async () => await unlisten.then((unlistenFn) => unlistenFn());
};
