import { workspaceService } from "@/lib/services/workbench/workspaceService";
import { useMutation } from "@tanstack/react-query";

export const USE_OPEN_WORKSPACE_QUERY_KEY = "openWorkspace";

const openWorkspaceFn = async (workspaceId: string): Promise<void> => {
  const result = await workspaceService.openWorkspace({
    id: workspaceId,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useOpenWorkspace = () => {
  return useMutation<void, Error, string>({
    mutationKey: [USE_OPEN_WORKSPACE_QUERY_KEY],
    mutationFn: openWorkspaceFn,
  });
};
