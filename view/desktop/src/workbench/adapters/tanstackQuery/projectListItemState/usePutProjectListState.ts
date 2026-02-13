import { projectListStateService } from "@/workbench/domains/projectListItemState/service";
import { ProjectListItemState } from "@/workbench/domains/projectListItemState/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_PROJECT_LIST_STATE_QUERY_KEY } from "./useGetProjectListState";

export const USE_PUT_PROJECT_LIST_STATE_MUTATION_KEY = "putProjectListState" as const;

export const usePutProjectListState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { projectListState: ProjectListItemState; workspaceId: string }>({
    mutationKey: [USE_PUT_PROJECT_LIST_STATE_MUTATION_KEY],
    mutationFn: ({ projectListState, workspaceId }) => projectListStateService.put(projectListState, workspaceId),
    onSuccess: (_, { projectListState, workspaceId }) => {
      queryClient.setQueryData([USE_GET_PROJECT_LIST_STATE_QUERY_KEY, workspaceId], projectListState);
    },
  });
};
