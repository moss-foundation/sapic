import { environmentListItemStateService } from "@/workbench/services/environmentListItemStateService";
import { useMutation } from "@tanstack/react-query";

export const USE_BATCH_REMOVE_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY = "batchRemoveEnvironmentListItemState" as const;

export const useBatchRemoveEnvironmentListItemState = () => {
  return useMutation<void, Error, { ids: string[]; workspaceId: string }>({
    mutationKey: [USE_BATCH_REMOVE_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ ids, workspaceId }) => environmentListItemStateService.batchRemoveExpanded(ids, workspaceId),
  });
};
