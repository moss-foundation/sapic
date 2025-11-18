import { StreamEnvironmentsResult } from "@/domains/environment/ipc";
import { environmentIpc } from "@/infra/ipc/environment";
import { BatchUpdateEnvironmentGroupInput, BatchUpdateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const BATCH_UPDATE_ENVIRONMENT_GROUP_MUTATION_KEY = "batchUpdateEnvironmentGroup";

export const useBatchUpdateEnvironmentGroup = () => {
  const queryClient = useQueryClient();

  return useMutation<BatchUpdateEnvironmentOutput, Error, BatchUpdateEnvironmentGroupInput>({
    mutationKey: [BATCH_UPDATE_ENVIRONMENT_GROUP_MUTATION_KEY],
    mutationFn: (input) => environmentIpc.batchUpdateEnvironmentGroup(input),
    onSuccess: (_, variables) => {
      queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsResult) => {
        return {
          ...old,
          groups: old.groups.map((group) => ({
            ...group,
            expanded: variables.items.find((item) => item.projectId === group.projectId)?.expanded ?? group.expanded,
            order: variables.items.find((item) => item.projectId === group.projectId)?.order ?? group.order,
            environments: old.environments.filter((environment) => environment.projectId === group.projectId),
          })),
        };
      });
    },
  });
};
