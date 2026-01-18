import { environmentService } from "@/domains/environment/environmentService";
import { ActivateEnvironmentInput, ActivateEnvironmentOutput, StreamEnvironmentsEvent } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useAllStreamedProjectEnvironments } from "./derived/useAllStreamedProjectEnvironments";
import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY, useStreamEnvironments } from "./useStreamEnvironments";

const ACTIVATE_ENVIRONMENT_QUERY_KEY = "activateEnvironment" as const;

export const useActivateEnvironment = () => {
  const queryClient = useQueryClient();

  const { data: workspaceEnvironments } = useStreamEnvironments();
  const { allProjectEnvironments } = useAllStreamedProjectEnvironments();

  return useMutation<ActivateEnvironmentOutput, Error, ActivateEnvironmentInput>({
    mutationKey: [ACTIVATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: (input) => environmentService.activateEnvironment(input),
    onSuccess: (data) => {
      if (workspaceEnvironments?.some((environment) => environment.id === data.environmentId)) {
        queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsEvent[]) => {
          return old.map((environment) => {
            if (environment.projectId !== null) return environment;
            return {
              ...environment,
              isActive: environment.id === data.environmentId,
            };
          });
        });
      }

      const projectEnvironment = allProjectEnvironments?.find((environment) => environment.id === data.environmentId);
      if (projectEnvironment) {
        queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsEvent[]) => {
          return old.map((environment) => {
            if (environment.projectId !== projectEnvironment?.projectId) return environment;
            return {
              ...environment,
              isActive: environment.id === data.environmentId,
            };
          });
        });
      }
    },
  });
};
