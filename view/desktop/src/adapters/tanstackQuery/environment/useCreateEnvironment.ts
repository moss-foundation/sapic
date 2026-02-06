import { environmentService } from "@/domains/environment/environmentService";
import { CreateEnvironmentInput, CreateEnvironmentOutput, StreamEnvironmentsEvent } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

export const useCreateEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation<CreateEnvironmentOutput, Error, CreateEnvironmentInput>({
    mutationFn: (input) => environmentService.createEnvironment(input),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
      if (!data.projectId) {
        queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsEvent[]) => {
          const newEnv = {
            ...data,
            isActive: false,
            totalVariables: 0,
          } satisfies StreamEnvironmentsEvent;

          return [...old, newEnv];
        });
      }
      return data;
    },
  });
};
