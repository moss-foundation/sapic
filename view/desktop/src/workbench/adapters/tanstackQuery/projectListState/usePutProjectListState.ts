import { projectListStateService } from "@/workbench/services/projectListStateService";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_PROJECT_LIST_STATE_QUERY_KEY } from "./useGetProjectListState";

export const USE_PUT_PROJECT_LIST_STATE_MUTATION_KEY = "putProjectListState" as const;

export const usePutProjectListState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { expanded: boolean; workspaceId: string }>({
    mutationKey: [USE_PUT_PROJECT_LIST_STATE_MUTATION_KEY],
    mutationFn: ({ expanded, workspaceId }) => projectListStateService.put(expanded, workspaceId),
    onSuccess: (_, { expanded, workspaceId }) => {
      queryClient.setQueryData([USE_GET_PROJECT_LIST_STATE_QUERY_KEY, workspaceId], expanded);
    },
  });
};
