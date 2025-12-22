import { workspaceIpc } from "@/infra/ipc/workspaceIpc";
import { DeleteWorkspaceInput, DeleteWorkspaceOutput, ListWorkspacesOutput } from "@repo/ipc";

export interface IWorkspaceService {
  list: () => Promise<ListWorkspacesOutput>;
  delete: (input: DeleteWorkspaceInput) => Promise<DeleteWorkspaceOutput>;
}

export const workspaceService: IWorkspaceService = {
  list: async () => {
    return await workspaceIpc.listWorkspaces();
  },
  delete: async (input: DeleteWorkspaceInput) => {
    return await workspaceIpc.deleteWorkspace(input);
  },
};
