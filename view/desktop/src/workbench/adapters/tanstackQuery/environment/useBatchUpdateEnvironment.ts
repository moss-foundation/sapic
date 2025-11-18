import { StreamEnvironmentsResult } from "@/domains/environment/ipc";
import { environmentIpc } from "@/infra/ipc/environment";
import { BatchUpdateEnvironmentInput, BatchUpdateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const BATCH_UPDATE_ENVIRONMENT_QUERY_KEY = "batchUpdateEnvironment";

export const useBatchUpdateEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation<BatchUpdateEnvironmentOutput, Error, BatchUpdateEnvironmentInput>({
    mutationKey: [BATCH_UPDATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: (input) => environmentIpc.batchUpdateEnvironment(input),
    onSuccess: (_, variables) => {
      queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsResult) => {
        return {
          ...old,
          environments: old.environments.map((oldEnv) => {
            const updatedEnv = variables.items.find((updatedEnv) => updatedEnv.id === oldEnv.id);
            if (updatedEnv) {
              return {
                ...oldEnv,
                order: updatedEnv.order,
              };
            }

            return oldEnv;
          }),
        };
      });
    },
  });
};
