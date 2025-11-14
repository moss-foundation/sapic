import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { BatchUpdateEnvironmentInput, BatchUpdateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { StreamEnvironmentsResult, USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const BATCH_UPDATE_ENVIRONMENT_QUERY_KEY = "batchUpdateEnvironment";

const batchUpdateEnvironment = async (input: BatchUpdateEnvironmentInput) => {
  const result = await invokeTauriIpc<BatchUpdateEnvironmentOutput>("batch_update_environment", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useBatchUpdateEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: [BATCH_UPDATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: batchUpdateEnvironment,
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
