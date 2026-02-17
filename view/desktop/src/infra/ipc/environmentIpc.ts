import { IEnvironmentIpc } from "@/domains/environment";
import {
  BatchUpdateEnvironmentInput,
  BatchUpdateEnvironmentOutput,
  UpdateEnvironmentInput,
  UpdateEnvironmentOutput,
} from "@repo/ipc";

import { invokeTauriServiceIpc } from "./tauri";

export const environmentIpc: IEnvironmentIpc = {
  listWorkspaceEnvironments: async () => {
    return await invokeTauriServiceIpc("main__list_workspace_environments");
  },
  listProjectEnvironments: async (input) => {
    return await invokeTauriServiceIpc("main__list_project_environments", { input });
  },
  activateEnvironment: async (input) => {
    return await invokeTauriServiceIpc("activate_environment", { input });
  },
  createEnvironment: async (input) => {
    return await invokeTauriServiceIpc("create_environment", { input });
  },
  deleteEnvironment: async (input) => {
    return await invokeTauriServiceIpc("delete_environment", { input });
  },
  updateEnvironment: (async (input: UpdateEnvironmentInput | BatchUpdateEnvironmentInput) => {
    if ("items" in input) {
      return await invokeTauriServiceIpc<BatchUpdateEnvironmentOutput>("batch_update_environment", { input });
    }

    return await invokeTauriServiceIpc<UpdateEnvironmentOutput>("update_environment", { input });
  }) as IEnvironmentIpc["updateEnvironment"],
};
