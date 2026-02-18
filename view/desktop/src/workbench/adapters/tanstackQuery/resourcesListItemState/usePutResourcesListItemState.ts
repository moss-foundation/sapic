import { resourcesListItemStateService } from "@/workbench/services/resourcesListItemStateService";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY } from "./useGetResourcesListItemState";

export const USE_PUT_RESOURCES_LIST_ITEM_STATE_MUTATION_KEY = "putResourcesListItem State" as const;

export const usePutResourcesListItemState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { expanded: boolean; resourcesListItemId: string; workspaceId: string }>({
    mutationKey: [USE_PUT_RESOURCES_LIST_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ expanded, resourcesListItemId, workspaceId }) =>
      resourcesListItemStateService.put(resourcesListItemId, expanded, workspaceId),
    onSuccess: (_, { expanded, resourcesListItemId, workspaceId }) => {
      queryClient.setQueryData(
        [USE_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY, resourcesListItemId, workspaceId],
        expanded
      );
    },
  });
};
