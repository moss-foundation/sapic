import { environmentService } from "@/domains/environment/environmentService";
import { StreamEnvironmentsEvent, UpdateEnvironmentGroupInput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const UPDATE_ENVIRONMENT_GROUP_QUERY_KEY = "updateEnvironmentGroup";

export const useUpdateEnvironmentGroup = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, UpdateEnvironmentGroupInput>({
    mutationKey: [UPDATE_ENVIRONMENT_GROUP_QUERY_KEY],
    mutationFn: (input) => environmentService.updateEnvironmentGroup(input),
    onSuccess: (_, variables) => {
      queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsEvent[]) => {
        return old.map((environment) => {
          if (environment.projectId === variables.projectId) {
            return {
              ...environment,
              ...variables,
            };
          }
          return environment;
        });
      });
    },
  });
};
