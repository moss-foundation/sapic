import { workspaceService } from "@/lib/services/workbench/workspaceService";
import { useMutation } from "@tanstack/react-query";

export const USE_CLOSE_WORKSPACE_QUERY_KEY = "closeWorkspace";

const closeWorkspaceFn = async (): Promise<void> => {
  const result = await workspaceService.closeWorkspace();

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCloseWorkspace = () => {
  return useMutation<void, Error, string>({
    mutationKey: [USE_CLOSE_WORKSPACE_QUERY_KEY],
    mutationFn: closeWorkspaceFn,
  });
};
