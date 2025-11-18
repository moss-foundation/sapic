import { StreamEnvironmentsResult } from "@/domains/environment/ipc";
import { environmentIpc } from "@/infra/ipc/environment";
import { useGroupedEnvironments } from "@/workbench/ui/components/EnvironmentsLists/hooks/useGroupedEnvironments";
import { ActivateEnvironmentInput, ActivateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY, useStreamEnvironments } from "./useStreamEnvironments";

const ACTIVATE_ENVIRONMENT_QUERY_KEY = "activateEnvironment" as const;

export const useActivateEnvironment = () => {
  const queryClient = useQueryClient();

  const { globalEnvironments } = useStreamEnvironments();
  const { groupedEnvironments } = useGroupedEnvironments();

  return useMutation<ActivateEnvironmentOutput, Error, ActivateEnvironmentInput>({
    mutationKey: [ACTIVATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: (input) => environmentIpc.activateEnvironment(input),
    onSuccess: (data) => {
      if (globalEnvironments.some((environment) => environment.id === data.environmentId)) {
        queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsResult) => {
          return {
            ...old,
            environments: old.environments.map((environment) => {
              if (environment.projectId !== null) return environment;
              return {
                ...environment,
                isActive: environment.id === data.environmentId,
              };
            }),
          };
        });
      }

      const groupedEnvironment = groupedEnvironments?.find((group) =>
        group.environments.some((env) => env.id === data.environmentId)
      );
      if (groupedEnvironment) {
        queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsResult) => {
          return {
            ...old,
            environments: old.environments.map((environment) => {
              if (environment.projectId !== groupedEnvironment?.projectId) return environment;

              return {
                ...environment,
                isActive: environment.id === data.environmentId,
              };
            }),
          };
        });
      }
    },
  });
};
