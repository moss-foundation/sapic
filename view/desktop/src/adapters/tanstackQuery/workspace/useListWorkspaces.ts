import { workspaceService } from "@/domains/workspace/workspaceService";
import { ListWorkspacesOutput } from "@repo/ipc";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_WORKSPACES_QUERY_KEY = "listWorkspaces";

const listWorkspacesFn = async (): Promise<ListWorkspacesOutput> => {
  return await workspaceService.list();
};

export const useListWorkspaces = () => {
  return useQuery<ListWorkspacesOutput, Error>({
    queryKey: [USE_LIST_WORKSPACES_QUERY_KEY],
    queryFn: listWorkspacesFn,
    placeholderData: [],
  });
};
