import {
  DeleteWorkspaceInput,
  DeleteWorkspaceOutput,
  ListWorkspacesOutput,
  MainWindow_CreateWorkspaceInput,
  MainWindow_CreateWorkspaceOutput,
  MainWindow_OpenWorkspaceInput,
  MainWindow_UpdateWorkspaceInput,
  MainWindow_UpdateWorkspaceOutput,
  WelcomeWindow_CreateWorkspaceInput,
  WelcomeWindow_CreateWorkspaceOutput,
  WelcomeWindow_OpenWorkspaceInput,
  WelcomeWindow_UpdateWorkspaceInput,
  WelcomeWindow_UpdateWorkspaceOutput,
} from "@repo/ipc";

export interface IWorkspaceIpc {
  listWorkspaces: () => Promise<ListWorkspacesOutput>;
  deleteWorkspace: (input: DeleteWorkspaceInput) => Promise<DeleteWorkspaceOutput>;

  main__openWorkspace: (input: MainWindow_OpenWorkspaceInput) => Promise<void>;
  main__createWorkspace: (input: MainWindow_CreateWorkspaceInput) => Promise<MainWindow_CreateWorkspaceOutput>;
  main__closeWorkspace: () => Promise<void>;
  main__updateWorkspace: (input: MainWindow_UpdateWorkspaceInput) => Promise<MainWindow_UpdateWorkspaceOutput>;

  welcome__createWorkspace: (input: WelcomeWindow_CreateWorkspaceInput) => Promise<WelcomeWindow_CreateWorkspaceOutput>;
  welcome__openWorkspace: (input: WelcomeWindow_OpenWorkspaceInput) => Promise<void>;
  welcome__updateWorkspace: (input: WelcomeWindow_UpdateWorkspaceInput) => Promise<WelcomeWindow_UpdateWorkspaceOutput>;
}
