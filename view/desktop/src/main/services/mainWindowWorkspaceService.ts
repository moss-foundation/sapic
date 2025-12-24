import { IWorkspaceService, workspaceService } from "@/domains/workspace/workspaceService";
import { workspaceIpc } from "@/infra/ipc/workspaceIpc";
import {
  MainWindow_CreateWorkspaceInput,
  MainWindow_CreateWorkspaceOutput,
  MainWindow_OpenWorkspaceInput,
  MainWindow_UpdateWorkspaceInput,
  MainWindow_UpdateWorkspaceOutput,
} from "@repo/ipc";

interface IMainWorkspaceService extends IWorkspaceService {
  create: (input: MainWindow_CreateWorkspaceInput) => Promise<MainWindow_CreateWorkspaceOutput>;
  open: (input: MainWindow_OpenWorkspaceInput) => Promise<void>;
  update: (input: MainWindow_UpdateWorkspaceInput) => Promise<MainWindow_UpdateWorkspaceOutput>;
  close: () => Promise<void>;
}

export const mainWorkspaceService: IMainWorkspaceService = {
  ...workspaceService,
  create: async (input) => {
    return await workspaceIpc.main__createWorkspace(input);
  },
  open: async (input) => {
    return await workspaceIpc.main__openWorkspace(input);
  },
  close: async () => {
    return await workspaceIpc.main__closeWorkspace();
  },
  update: async (input) => {
    return await workspaceIpc.main__updateWorkspace(input);
  },
};
