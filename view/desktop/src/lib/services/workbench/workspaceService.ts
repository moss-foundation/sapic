import { invokeTauriServiceIpc } from "@/lib/backend/tauri";
import {
  CloseWorkspaceInput,
  CloseWorkspaceOutput,
  CreateWorkspaceInput,
  CreateWorkspaceOutput,
  DeleteWorkspaceInput,
  DeleteWorkspaceOutput,
  ListWorkspacesOutput,
  OpenWorkspaceInput,
  OpenWorkspaceOutput,
  UpdateWorkspaceInput,
} from "@repo/window";

export const workspaceService = {
  createWorkspace: async (input: CreateWorkspaceInput) => {
    return await invokeTauriServiceIpc<CreateWorkspaceInput, CreateWorkspaceOutput>({
      cmd: "create_workspace",
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
      cmd: "close_workspace",
      args: {
        input,
      },
    });
  },

  updateWorkspace: async (input: UpdateWorkspaceInput) => {
    return await invokeTauriServiceIpc<UpdateWorkspaceInput, void>({
      cmd: "update_workspace",
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
