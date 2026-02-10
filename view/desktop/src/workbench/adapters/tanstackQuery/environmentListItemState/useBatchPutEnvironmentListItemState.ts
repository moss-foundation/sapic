import { environmentListItemStateService } from "@/workbench/domains/environmentListItemState/service";
import { EnvironmentListItemState } from "@/workbench/domains/environmentListItemState/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY } from "./useBatchGetEnvironmentListItemState";

export const USE_BATCH_PUT_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY = "batchPutEnvironmentListItemState" as const;

export const useBatchPutEnvironmentListItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { environmentListItemStates: EnvironmentListItemState[]; workspaceId: string }>({
    mutationKey: [USE_BATCH_PUT_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ environmentListItemStates, workspaceId }) =>
      environmentListItemStateService.batchPut(environmentListItemStates, workspaceId),
    onSuccess: (_, { environmentListItemStates, workspaceId }) => {
      environmentListItemStates.forEach((environmentListItemState) => {
        queryClient.setQueryData(
          [USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY, environmentListItemState.id, workspaceId],
          environmentListItemState
        );
      });
    },
  });
};
