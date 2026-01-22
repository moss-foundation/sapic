import { environmentService } from "@/domains/environment/environmentService";
import { DeleteEnvironmentInput, DeleteEnvironmentOutput, StreamEnvironmentsEvent } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

export const useDeleteEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation<DeleteEnvironmentOutput, Error, DeleteEnvironmentInput>({
    mutationFn: (input) => environmentService.deleteEnvironment(input),
    onSuccess: (data) => {
      queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsEvent[]) => {
        return old.filter((environment) => environment.id !== data.id);
      });
    },
  });
};
