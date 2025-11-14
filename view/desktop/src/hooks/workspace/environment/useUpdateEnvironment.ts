import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { UpdateEnvironmentInput, UpdateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { StreamEnvironmentsResult, USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const UPDATE_ENVIRONMENT_QUERY_KEY = "updateEnvironment";

const updateEnvironment = async (input: UpdateEnvironmentInput) => {
  const result = await invokeTauriIpc<UpdateEnvironmentOutput>("update_environment", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: [UPDATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: updateEnvironment,
    onSuccess: (data, variables) => {
      queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsResult) => {
        return {
          ...old,
          environments: old.environments.map((environment) =>
            environment.id === variables.id
              ? {
                  ...environment,
                  ...data,
                  ...variables,
                }
              : environment
          ),
        };
      });
    },
  });
};
