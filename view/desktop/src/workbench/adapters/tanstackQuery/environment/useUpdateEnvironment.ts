import { StreamEnvironmentsResult } from "@/domains/environment/ipc";
import { environmentIpc } from "@/infra/ipc/environment";
import { UpdateEnvironmentInput, UpdateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const UPDATE_ENVIRONMENT_QUERY_KEY = "updateEnvironment";

export const useUpdateEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation<UpdateEnvironmentOutput, Error, UpdateEnvironmentInput>({
    mutationKey: [UPDATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: (input) => environmentIpc.updateEnvironment(input),
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
