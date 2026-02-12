import { workspaceListStateService } from "@/workbench/domains/workspaceListItemState/service";
import { WorkspaceListState } from "@/workbench/domains/workspaceListItemState/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_WORKSPACE_LIST_STATE_QUERY_KEY } from "./useGetWorkspaceListState";

export const USE_PUT_WORKSPACE_LIST_STATE_MUTATION_KEY = "putWorkspaceListState" as const;

export const usePutWorkspaceListState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { workspaceListState: WorkspaceListState; workspaceId: string }>({
    mutationKey: [USE_PUT_WORKSPACE_LIST_STATE_MUTATION_KEY],
    mutationFn: ({ workspaceListState, workspaceId }) => workspaceListStateService.put(workspaceListState, workspaceId),
    onSuccess: (_, { workspaceListState, workspaceId }) => {
      queryClient.setQueryData([USE_GET_WORKSPACE_LIST_STATE_QUERY_KEY, workspaceId], workspaceListState);
    },
  });
};
