import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ActivateEnvironmentInput, ActivateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { StreamEnvironmentsResult, USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

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

  return useMutation({
    mutationKey: [ACTIVATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: activateEnvironment,
    onSuccess: (data) => {
      queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsResult) => {
        return {
          ...old,
          environments: old.environments.map((environment) => ({
            ...environment,
            isActive: environment.id === data.environmentId,
          })),
        };
      });
    },
  });
};
