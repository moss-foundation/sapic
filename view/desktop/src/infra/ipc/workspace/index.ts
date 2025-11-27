import { IWorkspaceIpc } from "@/domains/workspace";
import { invoke } from "@tauri-apps/api/core";

export const workspaceIpc: IWorkspaceIpc = {
  listWorkspaces: async () => {
    return await invoke("list_workspaces");
  },
  deleteWorkspace: async (input) => {
    return await invoke("delete_workspace", { input });
  },

  main__openWorkspace: async (input) => {
    return await invoke("main__open_workspace", { input });
  },
  main__createWorkspace: async (input) => {
    return await invoke("main__create_workspace", { input });
  },
  main__closeWorkspace: async () => {
    return await invoke("main__close_workspace");
  },
  main__updateWorkspace: async (input) => {
    return await invoke("main__update_workspace", { input });
  },

  welcome__createWorkspace: async (input) => {
    return await invoke("welcome__create_workspace", { input });
  },
  welcome__openWorkspace: async (input) => {
    return await invoke("welcome__open_workspace", { input });
  },
  welcome__updateWorkspace: async (input) => {
    return await invoke("welcome__update_workspace", { input });
  },
};
