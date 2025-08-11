import { invokeTauriIpc } from "@/lib/backend/tauri";
import { UpdateEnvironmentInput, UpdateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const updateEnvironment = async (input: UpdateEnvironmentInput) => {
  const result = await invokeTauriIpc<UpdateEnvironmentOutput>("update_environment", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: updateEnvironment,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
    },
  });
};
