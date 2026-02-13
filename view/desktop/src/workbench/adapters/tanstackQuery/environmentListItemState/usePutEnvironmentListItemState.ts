import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";
import { environmentListItemStateService } from "@/workbench/domains/environmentListItemState/service";
import { EnvironmentListItemState } from "@/workbench/domains/environmentListItemState/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY } from "./useGetEnvironmentListItemState";

export const USE_PUT_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY = "putEnvironmentListItemState" as const;

export const usePutEnvironmentListItemState = () => {
  const queryClient = useQueryClient();
  const { data: projects } = useListProjects();

  return useMutation<void, Error, { environmentListItemState: EnvironmentListItemState; workspaceId: string }>({
    mutationKey: [USE_PUT_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ environmentListItemState, workspaceId }) =>
      environmentListItemStateService.put(environmentListItemState, workspaceId),
    onSuccess: (_, { environmentListItemState, workspaceId }) => {
      queryClient.setQueryData(
        [USE_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY, environmentListItemState.id, workspaceId],
        environmentListItemState
      );

      const ids = projects?.items.map((project) => project.id) ?? [];
      queryClient.setQueryData(
        [USE_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY, ids, workspaceId],
        projects?.items.map((project): EnvironmentListItemState => {
          return {
            id: project.id,
            expanded: environmentListItemState.id === project.id ? environmentListItemState.expanded : false,
          };
        }) ?? []
      );
    },
  });
};
