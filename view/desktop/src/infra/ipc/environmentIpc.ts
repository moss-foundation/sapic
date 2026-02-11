import { IEnvironmentIpc } from "@/domains/environment";
import { invoke } from "@tauri-apps/api/core";

export const environmentIpc: IEnvironmentIpc = {
  listWorkspaceEnvironments: async () => {
    return await invoke("main__list_workspace_environments");
  },
  listProjectEnvironments: async (input) => {
    return await invoke("main__list_project_environments", { input });
  },
  activateEnvironment: async (input) => {
    return await invoke("activate_environment", { input });
  },
  batchUpdateEnvironment: async (input) => {
    return await invoke("batch_update_environment", { input });
  },
  createEnvironment: async (input) => {
    return await invoke("create_environment", { input });
  },
  deleteEnvironment: async (input) => {
    return await invoke("delete_environment", { input });
  },
  updateEnvironment: async (input) => {
    return await invoke("update_environment", { input });
  },
};
