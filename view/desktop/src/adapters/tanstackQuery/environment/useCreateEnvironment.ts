import { CreateEnvironmentParams, environmentService } from "@/domains/environment/environmentService";
import { CreateEnvironmentOutput, ListEnvironmentItem, ListWorkspaceEnvironmentsOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY } from "./useListWorkspaceEnvironments";

export const useCreateEnvironment = () => {
  const queryClient = useQueryClient();
  return useMutation<CreateEnvironmentOutput, Error, CreateEnvironmentParams>({
    mutationFn: (input) => environmentService.createEnvironment(input),
    onSuccess: (data) => {
      queryClient.setQueryData(
        [USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY],
        (old: ListWorkspaceEnvironmentsOutput | undefined) => {
          const newEnvironment: ListEnvironmentItem = {
            ...data,
            isActive: true,
            totalVariables: 0,
          };

          return {
            items: [...(old?.items ?? []), newEnvironment],
          } satisfies ListWorkspaceEnvironmentsOutput;
        }
      );
    },
  });
};
