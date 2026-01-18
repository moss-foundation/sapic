import { environmentService } from "@/domains/environment/environmentService";
import { BatchUpdateEnvironmentGroupInput, BatchUpdateEnvironmentOutput, StreamEnvironmentsEvent } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const BATCH_UPDATE_ENVIRONMENT_GROUP_MUTATION_KEY = "batchUpdateEnvironmentGroup";

export const useBatchUpdateEnvironmentGroup = () => {
  const queryClient = useQueryClient();

  return useMutation<BatchUpdateEnvironmentOutput, Error, BatchUpdateEnvironmentGroupInput>({
    mutationKey: [BATCH_UPDATE_ENVIRONMENT_GROUP_MUTATION_KEY],
    mutationFn: (input) => environmentService.batchUpdateEnvironmentGroup(input),
    onSuccess: (_, variables) => {
      queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsEvent[]) => {
        return old.map((environment) => ({
          ...environment,
          order: variables.items.find((item) => item.projectId === environment.projectId)?.order ?? environment.order,
        }));
      });
    },
  });
};
