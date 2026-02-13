import { environmentService } from "@/domains/environment/environmentService";
import { DeleteEnvironmentInput, DeleteEnvironmentOutput, ListWorkspaceEnvironmentsOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY } from "./useListWorkspaceEnvironments";

export const useDeleteEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation<DeleteEnvironmentOutput, Error, DeleteEnvironmentInput>({
    mutationFn: (input) => environmentService.deleteEnvironment(input),
    onSuccess: (data, variables) => {
      if (!variables.projectId) {
        queryClient.setQueryData(
          [USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY],
          (old: ListWorkspaceEnvironmentsOutput | undefined) => {
            if (!old) return old;
            return { ...old, items: old.items.filter((environment) => environment.id !== data.id) };
          }
        );
      }
    },
  });
};
