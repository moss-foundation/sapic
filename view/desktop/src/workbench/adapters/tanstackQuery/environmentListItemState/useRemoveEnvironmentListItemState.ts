import { environmentListItemStateService } from "@/workbench/domains/environmentListItemState/service";
import { useMutation } from "@tanstack/react-query";

export const USE_REMOVE_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY = "removeEnvironmentListItemState" as const;

export const useRemoveEnvironmentListItemState = () => {
  return useMutation<void, Error, { id: string; workspaceId: string }>({
    mutationKey: [USE_REMOVE_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ id, workspaceId }) => environmentListItemStateService.remove(id, workspaceId),
  });
};
