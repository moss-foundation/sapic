import { IWorkspaceIpc } from "@/domains/workspace";
import { invoke } from "@tauri-apps/api/core";

export const workspaceIpc: IWorkspaceIpc = {
  listWorkspaces: async () => {
    return await invoke("list_workspaces");
  },
  deleteWorkspace: async (input) => {
    return await invoke("delete_workspace", { input });
  },

  main_openWorkspace: async (input) => {
    return await invoke("main__open_workspace", { input });
  },
  main_createWorkspace: async (input) => {
    return await invoke("main__create_workspace", { input });
  },
  main_closeWorkspace: async () => {
    return await invoke("main__close_workspace");
  },
  main_updateWorkspace: async (input) => {
    return await invoke("main__update_workspace", { input });
  },

  welcome_createWorkspace: async (input) => {
    return await invoke("welcome__create_workspace", { input });
  },
  welcome_openWorkspace: async (input) => {
    return await invoke("welcome__open_workspace", { input });
  },
  welcome_updateWorkspace: async (input) => {
    return await invoke("welcome__update_workspace", { input });
  },
};
