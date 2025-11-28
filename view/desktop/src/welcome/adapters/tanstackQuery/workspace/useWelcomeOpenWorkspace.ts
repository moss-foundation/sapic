import { welcomeWindowWorkspaceService } from "@/welcome/services/welcomeWindowWorkspaceService";
import { WelcomeWindow_OpenWorkspaceInput } from "@repo/ipc";
import { useMutation } from "@tanstack/react-query";

export const USE_OPEN_WELCOME_WORKSPACE_QUERY_KEY = "openWelcomeWorkspace";

export const useWelcomeOpenWorkspace = () => {
  return useMutation<void, Error, WelcomeWindow_OpenWorkspaceInput>({
    mutationKey: [USE_OPEN_WELCOME_WORKSPACE_QUERY_KEY],
    mutationFn: welcomeWindowWorkspaceService.open,
  });
};
