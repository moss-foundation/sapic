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
  | "plugin:settings-storage|get_value"
  | "plugin:settings-storage|batch_get_value"
  | "plugin:settings-storage|update_value"
  | "plugin:settings-storage|remove_value"
  | "plugin:shared-storage|batch_put_item"
  | "plugin:shared-storage|batch_remove_item"
  | "plugin:shared-storage|batch_get_item"
  | "plugin:shared-storage|batch_get_item_by_prefix"
  | "plugin:shared-storage|batch_remove_item_by_prefix"
  | "plugin:template-parser|parse_url"
  //
  // App
  //
  | "list_workspaces"
  | "list_user_accounts"
  | "add_user_account"
  | "update_user_account"
  | "remove_user_account"
  | "list_configuration_schemas"
  | "execute_command"
  | "get_translation_namespace"
  | "describe_color_theme"
  | "list_languages"
  | "list_color_themes"
  | "list_extensions"
  | "download_extension"
  | "delete_workspace"
  | "update_workspace"
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
  // Onboarding
  //
  | "onboarding__complete_onboarding"
  //
  // Welcome
  //
  | "welcome__cancel_request"
  | "welcome__open_workspace"
  | "welcome__create_workspace"
  | "welcome__update_workspace"
  //
  // Workspace
  //
  | "describe_project"
  | "create_project"
  | "delete_project"
  | "main__list_projects"
  | "update_project"
  | "archive_project"
  | "unarchive_project"
  | "batch_update_project"
  | "list_changes"
  | "import_project"
  | "export_project"
  | "main__list_project_environments"
  | "main__list_workspace_environments"
  | "main__list_project_resources"
  | "create_environment"
  | "update_environment"
  | "batch_update_environment"
  | "delete_environment"
  | "activate_environment"
  | "describe_environment"
  //
  // Project
  //
  | "create_project_resource"
  | "delete_project_resource"
  | "update_project_resource"
  | "describe_project_resource"
  | "batch_update_project_resource"
  | "batch_create_project_resource"
  | "execute_vcs_operation";

export const invokeTauriServiceIpc = async <T = unknown>(cmd: TauriIpcCommand, args?: InvokeArgs): Promise<T> => {
  return await invokeTauri<T>(cmd, args);
};
