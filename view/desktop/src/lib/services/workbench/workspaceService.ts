import { invokeTauriServiceIpc } from "@/infra/ipc/tauri";
import {
  DeleteWorkspaceInput,
  DeleteWorkspaceOutput,
  ListWorkspacesOutput,
  MainWindow_CreateWorkspaceInput,
  MainWindow_CreateWorkspaceOutput,
  MainWindow_UpdateWorkspaceInput,
} from "@repo/ipc";
import { CloseWorkspaceInput, CloseWorkspaceOutput, OpenWorkspaceInput, OpenWorkspaceOutput } from "@repo/window";

export const workspaceService = {
  createWorkspace: async (input: MainWindow_CreateWorkspaceInput) => {
    return await invokeTauriServiceIpc<MainWindow_CreateWorkspaceInput, MainWindow_CreateWorkspaceOutput>({
      cmd: "main__create_workspace",
      args: {
        input,
      },
    });
  },
  deleteWorkspace: async (input: DeleteWorkspaceInput) => {
    return await invokeTauriServiceIpc<DeleteWorkspaceInput, DeleteWorkspaceOutput>({
      cmd: "delete_workspace",
      args: {
        input,
      },
    });
  },

  openWorkspace: async (input: OpenWorkspaceInput) => {
    return await invokeTauriServiceIpc<OpenWorkspaceInput, OpenWorkspaceOutput>({
      cmd: "welcome__open_workspace",
      args: {
        input,
      },
    });
  },
  closeWorkspace: async (input: CloseWorkspaceInput) => {
    return await invokeTauriServiceIpc<CloseWorkspaceInput, CloseWorkspaceOutput>({
      cmd: "main__close_workspace",
      args: {
        input,
      },
    });
  },

  updateWorkspace: async (input: MainWindow_UpdateWorkspaceInput) => {
    return await invokeTauriServiceIpc<MainWindow_UpdateWorkspaceInput, void>({
      cmd: "main__update_workspace",
      args: {
        input,
      },
    });
  },

  listWorkspaces: async () => {
    return await invokeTauriServiceIpc<void, ListWorkspacesOutput>({
      cmd: "list_workspaces",
    });
  },
};
