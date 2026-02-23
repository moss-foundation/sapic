import { IWorkspaceIpc } from "@/domains/workspace";

import { invokeTauriIpc } from "./tauri";

export const workspaceIpc: IWorkspaceIpc = {
  listWorkspaces: async () => {
    return await invokeTauriIpc("list_workspaces");
  },
  deleteWorkspace: async (input) => {
    return await invokeTauriIpc("delete_workspace", { input });
  },

  main__openWorkspace: async (input) => {
    return await invokeTauriIpc("main__open_workspace", { input });
  },
  main__createWorkspace: async (input) => {
    return await invokeTauriIpc("main__create_workspace", { input });
  },
  main__closeWorkspace: async () => {
    return await invokeTauriIpc("main__close_workspace");
  },
  main__updateWorkspace: async (input) => {
    return await invokeTauriIpc("main__update_workspace", { input });
  },

  welcome__createWorkspace: async (input) => {
    return await invokeTauriIpc("welcome__create_workspace", { input });
  },
  welcome__openWorkspace: async (input) => {
    return await invokeTauriIpc("welcome__open_workspace", { input });
  },
  welcome__updateWorkspace: async (input) => {
    return await invokeTauriIpc("welcome__update_workspace", { input });
  },
};
