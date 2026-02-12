import { workspaceListStateService } from "@/workbench/domains/workspaceListItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_WORKSPACE_LIST_STATE_QUERY_KEY } from "./useGetWorkspaceListState";

export const USE_REMOVE_WORKSPACE_LIST_STATE_MUTATION_KEY = "removeWorkspaceListState" as const;

export const useRemoveWorkspaceListState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { workspaceId: string }>({
    mutationKey: [USE_REMOVE_WORKSPACE_LIST_STATE_MUTATION_KEY],
    onSuccess: (_, { workspaceId }) => {
      queryClient.removeQueries({ queryKey: [USE_GET_WORKSPACE_LIST_STATE_QUERY_KEY, workspaceId] });
    },
    mutationFn: ({ workspaceId }) => workspaceListStateService.remove(workspaceId),
  });
};
