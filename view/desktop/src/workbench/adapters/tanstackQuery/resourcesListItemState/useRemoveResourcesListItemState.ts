import { resourcesListItemStateService } from "@/workbench/services/resourcesListItemStateService";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY } from "./useGetResourcesListItemState";

export const USE_REMOVE_RESOURCES_LIST_ITEM_STATE_MUTATION_KEY = "removeResourcesListItemState" as const;

export const useRemoveResourcesListItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { projectId: string; workspaceId: string }>({
    mutationKey: [USE_REMOVE_RESOURCES_LIST_ITEM_STATE_MUTATION_KEY],
    onSuccess: (_, { projectId, workspaceId }) => {
      queryClient.removeQueries({
        queryKey: [USE_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY, projectId, workspaceId],
      });
    },
    mutationFn: ({ projectId, workspaceId }) => resourcesListItemStateService.remove(projectId, workspaceId),
  });
};
