import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { environmentService } from "@/domains/environment/environmentService";
import { DeleteEnvironmentInput, DeleteEnvironmentOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY } from "./useListWorkspaceEnvironments";

export const useDeleteEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation<DeleteEnvironmentOutput, Error, DeleteEnvironmentInput>({
    mutationFn: (input) => environmentService.deleteEnvironment(input),
    onSuccess: (data, variables) => {
      if (!variables.projectId) {
        queryClient.setQueryData([USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY], (old: EnvironmentSummary[]) => {
          return old.filter((environment) => environment.id !== data.id);
        });
      }
    },
  });
};
