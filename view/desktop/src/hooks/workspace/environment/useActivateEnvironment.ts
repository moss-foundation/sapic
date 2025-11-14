import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useGroupedEnvironments } from "@/workbench/ui/components/EnvironmentsLists/hooks/useGroupedEnvironments";
import { ActivateEnvironmentInput, ActivateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { StreamEnvironmentsResult, USE_STREAMED_ENVIRONMENTS_QUERY_KEY, useStreamEnvironments } from "..";

const ACTIVATE_ENVIRONMENT_QUERY_KEY = "activateEnvironment" as const;

const activateEnvironment = async (input: ActivateEnvironmentInput) => {
  const result = await invokeTauriIpc<ActivateEnvironmentOutput>("activate_environment", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useActivateEnvironment = () => {
  const queryClient = useQueryClient();
  const { globalEnvironments } = useStreamEnvironments();
  const { groupedEnvironments } = useGroupedEnvironments();

  return useMutation({
    mutationKey: [ACTIVATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: activateEnvironment,
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
