import { resourcesListItemStateService } from "@/workbench/domains/resourcesListItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_RESOURCES_LIST_STATE_QUERY_KEY } from "./useGetResourcesListState";

export const USE_REMOVE_RESOURCES_LIST_STATE_MUTATION_KEY = "removeResourcesListState" as const;

export const useRemoveResourcesListState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { resourcesListItemId: string; workspaceId: string }>({
    mutationKey: [USE_REMOVE_RESOURCES_LIST_STATE_MUTATION_KEY],
    onSuccess: (_, { resourcesListItemId, workspaceId }) => {
      queryClient.removeQueries({
        queryKey: [USE_GET_RESOURCES_LIST_STATE_QUERY_KEY, resourcesListItemId, workspaceId],
      });
    },
    mutationFn: ({ resourcesListItemId, workspaceId }) =>
      resourcesListItemStateService.remove(resourcesListItemId, workspaceId),
  });
};
