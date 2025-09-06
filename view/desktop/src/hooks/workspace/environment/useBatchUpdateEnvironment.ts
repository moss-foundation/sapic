import { invokeTauriIpc } from "@/lib/backend/tauri";
import { BatchUpdateEnvironmentInput, BatchUpdateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

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
      //TODO: update the query data
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
      //   queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsResult) => {
      //     return {
      //       ...old,
      //       environments: variables.items.map((item) => ({
      //         ...old.environments.find((environment) => environment.id === item.id),
      //         ...item,
      //       })),
      //     };
      //   });
    },
  });
};
