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

  main_openWorkspace: (input: MainWindow_OpenWorkspaceInput) => Promise<void>;
  main_createWorkspace: (input: MainWindow_CreateWorkspaceInput) => Promise<MainWindow_CreateWorkspaceOutput>;
  main_closeWorkspace: () => Promise<void>;
  main_updateWorkspace: (input: MainWindow_UpdateWorkspaceInput) => Promise<MainWindow_UpdateWorkspaceOutput>;

  welcome_createWorkspace: (input: WelcomeWindow_CreateWorkspaceInput) => Promise<WelcomeWindow_CreateWorkspaceOutput>;
  welcome_openWorkspace: (input: WelcomeWindow_OpenWorkspaceInput) => Promise<void>;
  welcome_updateWorkspace: (input: WelcomeWindow_UpdateWorkspaceInput) => Promise<WelcomeWindow_UpdateWorkspaceOutput>;
}
