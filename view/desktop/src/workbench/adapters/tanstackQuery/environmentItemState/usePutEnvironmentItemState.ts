import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
import { EnvironmentItemState } from "@/workbench/domains/environmentItemState/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY } from "./useGetEnvironmentItemState";

export const USE_PUT_ENVIRONMENT_ITEM_STATE_MUTATION_KEY = "putEnvironmentItemState" as const;

export const usePutEnvironmentItemState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { environmentItemState: EnvironmentItemState; workspaceId: string }>({
    mutationKey: [USE_PUT_ENVIRONMENT_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ environmentItemState, workspaceId }) =>
      environmentItemStateService.put(environmentItemState, workspaceId),
    onSuccess: (_, { environmentItemState, workspaceId }) => {
      queryClient.setQueryData(
        [USE_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY, environmentItemState.id, workspaceId],
        environmentItemState
      );
    },
  });
};
