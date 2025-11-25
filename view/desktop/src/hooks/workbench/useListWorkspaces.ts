import { workspaceService } from "@/lib/services/workbench/workspaceService";
import { ListWorkspacesOutput } from "@repo/ipc";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_WORKSPACES_QUERY_KEY = "listWorkspaces";

const listWorkspacesFn = async (): Promise<ListWorkspacesOutput> => {
  const result = await workspaceService.listWorkspaces();

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useListWorkspaces = () => {
  return useQuery<ListWorkspacesOutput, Error>({
    queryKey: [USE_LIST_WORKSPACES_QUERY_KEY],
    queryFn: listWorkspacesFn,
    placeholderData: [],
  });
};
