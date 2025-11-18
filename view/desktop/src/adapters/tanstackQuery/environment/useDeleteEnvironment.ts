import { environmentService } from "@/domains/environment/environmentService";
import { StreamEnvironmentsResult } from "@/domains/environment/types";
import { DeleteEnvironmentInput, DeleteEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

export const useDeleteEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation<DeleteEnvironmentOutput, Error, DeleteEnvironmentInput>({
    mutationFn: (input) => environmentService.deleteEnvironment(input),
    onSuccess: (data) => {
      queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsResult) => {
        return {
          ...old,
          environments: old.environments.filter((environment) => environment.id !== data.id),
          groups: old.groups.filter((group) => {
            const envsWithoutDeleted = old.environments.filter((environment) => environment.id !== data.id);

            if (envsWithoutDeleted.some((environment) => environment.projectId === group.projectId)) {
              return true;
            }

            return false;
          }),
        };
      });
    },
  });
};
