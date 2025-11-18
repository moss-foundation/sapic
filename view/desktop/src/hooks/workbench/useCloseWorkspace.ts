import { workspaceService } from "@/lib/services/workbench/workspaceService";
import { CloseWorkspaceOutput } from "@repo/window";
import { useMutation } from "@tanstack/react-query";

export const USE_CLOSE_WORKSPACE_QUERY_KEY = "closeWorkspace";

const closeWorkspaceFn = async (workspaceId: string): Promise<CloseWorkspaceOutput> => {
  const result = await workspaceService.closeWorkspace({
    id: workspaceId,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCloseWorkspace = () => {
  return useMutation<CloseWorkspaceOutput, Error, string>({
    mutationKey: [USE_CLOSE_WORKSPACE_QUERY_KEY],
    mutationFn: closeWorkspaceFn,
  });
};
