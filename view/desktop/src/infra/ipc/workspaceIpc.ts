import { IWorkspaceIpc } from "@/domains/workspace";

import { invokeTauriServiceIpc } from "./tauri";

export const workspaceIpc: IWorkspaceIpc = {
  listWorkspaces: async () => {
    return await invokeTauriServiceIpc("list_workspaces");
  },
  deleteWorkspace: async (input) => {
    return await invokeTauriServiceIpc("delete_workspace", { input });
  },

  main__openWorkspace: async (input) => {
    return await invokeTauriServiceIpc("main__open_workspace", { input });
  },
  main__createWorkspace: async (input) => {
    return await invokeTauriServiceIpc("main__create_workspace", { input });
  },
  main__closeWorkspace: async () => {
    return await invokeTauriServiceIpc("main__close_workspace");
  },
  main__updateWorkspace: async (input) => {
    return await invokeTauriServiceIpc("main__update_workspace", { input });
  },

  welcome__createWorkspace: async (input) => {
    return await invokeTauriServiceIpc("welcome__create_workspace", { input });
  },
  welcome__openWorkspace: async (input) => {
    return await invokeTauriServiceIpc("welcome__open_workspace", { input });
  },
  welcome__updateWorkspace: async (input) => {
    return await invokeTauriServiceIpc("welcome__update_workspace", { input });
  },
};
