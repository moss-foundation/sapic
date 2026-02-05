import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
import { EnvironmentItemState } from "@/workbench/domains/environmentItemState/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY } from "./useGetEnvironmentItemState";

export const USE_BATCH_UPDATE_ENVIRONMENT_ITEM_STATE_MUTATION_KEY = "batchUpdateEnvironmentItemState" as const;

export const useBatchUpdateEnvironmentItemState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { environmentItemStates: EnvironmentItemState[]; workspaceId: string }>({
    mutationKey: [USE_BATCH_UPDATE_ENVIRONMENT_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ environmentItemStates, workspaceId }) =>
      environmentItemStateService.batchPut(environmentItemStates, workspaceId),
    onSuccess: (_, { environmentItemStates, workspaceId }) => {
      environmentItemStates.forEach((environmentItemState) => {
        queryClient.setQueryData(
          [USE_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY, environmentItemState.id, workspaceId],
          environmentItemState
        );
      });
    },
  });
};
