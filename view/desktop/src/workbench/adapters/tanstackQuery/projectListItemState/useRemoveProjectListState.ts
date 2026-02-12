import { projectListStateService } from "@/workbench/domains/projectListItemState/service";
import { useMutation } from "@tanstack/react-query";

export const USE_REMOVE_PROJECT_LIST_STATE_MUTATION_KEY = "removeProjectListState" as const;

export const useRemoveProjectListState = () => {
  return useMutation<void, Error, { workspaceId: string }>({
    mutationKey: [USE_REMOVE_PROJECT_LIST_STATE_MUTATION_KEY],
    mutationFn: ({ workspaceId }) => projectListStateService.remove(workspaceId),
  });
};
