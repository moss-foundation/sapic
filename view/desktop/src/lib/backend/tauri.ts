import { InvokeArgs, invoke as invokeTauri } from "@tauri-apps/api/core";
import type { EventCallback, EventName } from "@tauri-apps/api/event";
import { listen as listenTauri } from "@tauri-apps/api/event";

// Define all possible Tauri IPC commands as string literals
export type TauriIpcCommand =
  //
  // App
  //
  | "set_color_theme"
  | "set_locale"
  | "execute_command"
  | "get_translations"
  | "get_color_theme"
  | "list_locales"
  | "describe_app_state"
  | "list_color_themes"
  | "create_workspace"
  | "open_workspace"
  | "list_workspaces"
  | "delete_workspace"
  | "update_workspace"
  | "close_workspace"
  | "update_profile"
  //
  // Workspace
  //
  | "update_workspace_state"
  | "describe_workspace_state"
  | "list_collections"
  | "describe_collection"
  | "create_collection"
  | "delete_collection"
  | "stream_collections"
  | "update_collection"
  | "archive_collection"
  | "unarchive_collection"
  | "batch_update_collection"
  | "stream_environments"
  | "create_environment"
  | "update_environment"
  | "batch_update_environment"
  | "delete_environment"
  | "update_environment_group"
  | "batch_update_environment_group"
  | "activate_environment"
  //
  // Collection
  //
  | "create_collection_entry"
  | "delete_collection_entry"
  | "update_collection_entry"
  | "stream_collection_entries"
  | "batch_update_collection_entry"
  | "batch_create_collection_entry"
  | "import_collection"
  | "export_collection"
  //
  // Env
  //
  | "get_mistral_api_key";

export type IpcResult<T, E> = { status: "ok"; data: T } | { status: "error"; error: E };

export const handleTauriIpcError = (cmd: TauriIpcCommand, error: unknown) => {
  console.error(`Error in IPC command "${cmd}":`, error);

  // TODO: dispatch to a global error handler or show user notifications
};

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

export const listenTauriIpc = <T>(event: EventName, handle: EventCallback<T>) => {
  const unlisten = listenTauri(event, handle);
  return async () => await unlisten.then((unlistenFn) => unlistenFn());
};
