import { IEnvironmentIpc } from "@/domains/environment";
import { BatchUpdateEnvironmentOutput, UpdateEnvironmentOutput } from "@repo/ipc";

import { invokeTauriIpc } from "./tauri";

export const environmentIpc: IEnvironmentIpc = {
  listWorkspaceEnvironments: async () => {
    return await invokeTauriIpc("main__list_workspace_environments");
  },
  listProjectEnvironments: async (input) => {
    return await invokeTauriIpc("main__list_project_environments", { input });
  },
  activateEnvironment: async (input) => {
    return await invokeTauriIpc("activate_environment", { input });
  },
  createEnvironment: async (input) => {
    return await invokeTauriIpc("create_environment", { input });
  },
  deleteEnvironment: async (input) => {
    return await invokeTauriIpc("delete_environment", { input });
  },
  updateEnvironment: async (input) => {
    return await invokeTauriIpc<UpdateEnvironmentOutput>("update_environment", { input });
  },
  batchUpdateEnvironment: async (input) => {
    return await invokeTauriIpc<BatchUpdateEnvironmentOutput>("batch_update_environment", { input });
  },
};
