import { workspaceListStateService } from "@/workbench/domains/workspaceListItemState/service";
import { WorkspaceListState } from "@/workbench/domains/workspaceListItemState/types";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_WORKSPACE_LIST_STATE_QUERY_KEY = "getWorkspaceListState" as const;

export const useGetWorkspaceListState = (workspaceId: string) => {
  return useQuery<WorkspaceListState, Error>({
    queryKey: [USE_GET_WORKSPACE_LIST_STATE_QUERY_KEY, workspaceId],
    queryFn: () => workspaceListStateService.get(workspaceId),
  });
};
