import { environmentService } from "@/domains/environment/environmentService";
import { CreateEnvironmentInput, CreateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

export const useCreateEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation<CreateEnvironmentOutput, Error, CreateEnvironmentInput>({
    mutationFn: (input) => environmentService.createEnvironment(input),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });

      return data;
    },
  });
};
