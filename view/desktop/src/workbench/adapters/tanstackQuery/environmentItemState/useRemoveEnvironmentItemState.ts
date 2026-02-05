import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
import { useMutation } from "@tanstack/react-query";

export const USE_REMOVE_ENVIRONMENT_ITEM_STATE_MUTATION_KEY = "removeEnvironmentItemState" as const;

export const useRemoveEnvironmentItemState = () => {
  return useMutation<void, Error, { id: string; workspaceId: string }>({
    mutationKey: [USE_REMOVE_ENVIRONMENT_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ id, workspaceId }) => environmentItemStateService.remove(id, workspaceId),
  });
};
