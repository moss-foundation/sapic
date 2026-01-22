import { environmentService } from "@/domains/environment/environmentService";
import { StreamEnvironmentsEvent, UpdateEnvironmentInput, UpdateEnvironmentOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const UPDATE_ENVIRONMENT_QUERY_KEY = "updateEnvironment";

export const useUpdateEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation<UpdateEnvironmentOutput, Error, UpdateEnvironmentInput>({
    mutationKey: [UPDATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: (input) => environmentService.updateEnvironment(input),
    onSuccess: (data, variables) => {
      queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsEvent[]) => {
        return old.map((environment) =>
          environment.id === variables.id
            ? {
                ...environment,
                ...data,
                ...variables,
              }
            : environment
        );
      });
    },
  });
};
