import { activityBarItemStateService } from "@/workbench/domains/activityBarItemState/service";
import { useMutation } from "@tanstack/react-query";

export const USE_REMOVE_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY = "removeActivityBarItemState";

export const useRemoveActivityBarItemState = () => {
  return useMutation<void, Error, { id: string; workspaceId: string }>({
    mutationKey: [USE_REMOVE_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ id, workspaceId }) => activityBarItemStateService.remove(id, workspaceId),
  });
};
