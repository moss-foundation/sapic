import { environmentListItemStateService } from "@/workbench/services/environmentListItemStateService";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY } from "./useGetEnvironmentListItemState";

export const USE_PUT_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY = "putEnvironmentListItemState" as const;

export const usePutEnvironmentListItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { id: string; expanded: boolean; workspaceId: string }>({
    mutationKey: [USE_PUT_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ id, expanded, workspaceId }) =>
      environmentListItemStateService.putExpanded(id, expanded, workspaceId),
    onSuccess: (_, { id, expanded, workspaceId }) => {
      queryClient.setQueryData([USE_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY, id, workspaceId], expanded);
    },
  });
};
