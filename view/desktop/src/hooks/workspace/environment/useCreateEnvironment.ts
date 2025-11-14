import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { CreateEnvironmentInput, CreateEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const createEnvironment = async (input: CreateEnvironmentInput) => {
  const result = await invokeTauriIpc<CreateEnvironmentOutput>("create_environment", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createEnvironment,
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });

      return data;
    },
  });
};
