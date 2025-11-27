import { mainWorkspaceService } from "@/main/services/mainWindowWorkspaceService";
import { useMutation } from "@tanstack/react-query";

export const USE_CLOSE_WORKSPACE_QUERY_KEY = "closeWorkspace";

const closeWorkspaceFn = async (): Promise<void> => {
  return await mainWorkspaceService.close();
};

export const useCloseWorkspace = () => {
  return useMutation<void, Error, string>({
    mutationKey: [USE_CLOSE_WORKSPACE_QUERY_KEY],
    mutationFn: closeWorkspaceFn,
  });
};
