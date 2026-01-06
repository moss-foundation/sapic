import { IWorkspaceService, workspaceService } from "@/domains/workspace/workspaceService";
import { workspaceIpc } from "@/infra/ipc/workspaceIpc";
import {
  WelcomeWindow_CreateWorkspaceInput,
  WelcomeWindow_CreateWorkspaceOutput,
  WelcomeWindow_OpenWorkspaceInput,
  WelcomeWindow_UpdateWorkspaceInput,
  WelcomeWindow_UpdateWorkspaceOutput,
} from "@repo/ipc";

interface IWelcomeWindowWorkspaceService extends IWorkspaceService {
  create: (input: WelcomeWindow_CreateWorkspaceInput) => Promise<WelcomeWindow_CreateWorkspaceOutput>;
  open: (input: WelcomeWindow_OpenWorkspaceInput) => Promise<void>;
  update: (input: WelcomeWindow_UpdateWorkspaceInput) => Promise<WelcomeWindow_UpdateWorkspaceOutput>;
}

export const welcomeWindowWorkspaceService: IWelcomeWindowWorkspaceService = {
  ...workspaceService,
  create: async (input) => {
    return await workspaceIpc.welcome__createWorkspace(input);
  },
  open: async (input) => {
    return await workspaceIpc.welcome__openWorkspace(input);
  },
  update: async (input) => {
    return await workspaceIpc.welcome__updateWorkspace(input);
  },
};
