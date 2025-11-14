import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { DeleteEnvironmentInput, DeleteEnvironmentOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { StreamEnvironmentsResult, USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

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
