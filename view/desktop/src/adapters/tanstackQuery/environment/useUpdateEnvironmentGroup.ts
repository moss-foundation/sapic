import { environmentService } from "@/domains/environment/environmentService";
import { StreamEnvironmentsResult } from "@/domains/environment/types";
import { UpdateEnvironmentGroupInput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const UPDATE_ENVIRONMENT_GROUP_QUERY_KEY = "updateEnvironmentGroup";

export const useUpdateEnvironmentGroup = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, UpdateEnvironmentGroupInput>({
    mutationKey: [UPDATE_ENVIRONMENT_GROUP_QUERY_KEY],
    mutationFn: (input) => environmentService.updateEnvironmentGroup(input),
    onSuccess: (_, variables) => {
      queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsResult) => {
        return {
          ...old,
          groups: old.groups.map((group) => {
            if (group.projectId === variables.projectId) {
              return {
                ...group,
                ...variables,
              };
            }
            return group;
          }),
        };
      });
    },
  });
};
