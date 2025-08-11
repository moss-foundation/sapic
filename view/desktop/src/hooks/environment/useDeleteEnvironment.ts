import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DeleteEnvironmentInput, DeleteEnvironmentOutput, StreamEnvironmentsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const deleteEnvironment = async (input: DeleteEnvironmentInput) => {
  const result = await invokeTauriIpc<DeleteEnvironmentOutput>("delete_environment", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteEnvironment,
    onSuccess: (data) => {
      queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsEvent[]) => {
        return old.filter((environment) => environment.id !== data.id);
      });
    },
  });
};
