import { resourcesListItemStateService } from "@/workbench/domains/resourcesListItemState/service";
import { ResourcesListItemState } from "@/workbench/domains/resourcesListItemState/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_RESOURCES_LIST_STATE_QUERY_KEY } from "./useGetResourcesListState";

export const USE_PUT_RESOURCES_LIST_STATE_MUTATION_KEY = "putResourcesListState" as const;

export const usePutResourcesListState = () => {
  const queryClient = useQueryClient();
  return useMutation<
    void,
    Error,
    { resourcesListItemState: ResourcesListItemState; resourcesListItemId: string; workspaceId: string }
  >({
    mutationKey: [USE_PUT_RESOURCES_LIST_STATE_MUTATION_KEY],
    mutationFn: ({ resourcesListItemState, resourcesListItemId, workspaceId }) =>
      resourcesListItemStateService.put(resourcesListItemId, resourcesListItemState, workspaceId),
    onSuccess: (_, { resourcesListItemState, resourcesListItemId, workspaceId }) => {
      queryClient.setQueryData(
        [USE_GET_RESOURCES_LIST_STATE_QUERY_KEY, resourcesListItemId, workspaceId],
        resourcesListItemState
      );
    },
  });
};
