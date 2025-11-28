import { welcomeWindowWorkspaceService } from "@/welcome/services/welcomeWindowWorkspaceService";
import { WelcomeWindow_CreateWorkspaceInput, WelcomeWindow_CreateWorkspaceOutput } from "@repo/ipc";
import { useMutation } from "@tanstack/react-query";

export const USE_CREATE_WELCOME_WORKSPACE_QUERY_KEY = "createWelcomeWorkspace";

export const useWelcomeCreateWorkspace = () => {
  return useMutation<WelcomeWindow_CreateWorkspaceOutput, Error, WelcomeWindow_CreateWorkspaceInput>({
    mutationKey: [USE_CREATE_WELCOME_WORKSPACE_QUERY_KEY],
    mutationFn: (input) => welcomeWindowWorkspaceService.create(input),
  });
};
