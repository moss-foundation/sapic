import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { UpdateEnvironmentGroupInput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { StreamEnvironmentsResult, USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";

const UPDATE_ENVIRONMENT_GROUP_QUERY_KEY = "updateEnvironmentGroup";

const updateEnvironmentGroup = async (input: UpdateEnvironmentGroupInput) => {
  const result = await invokeTauriIpc("update_environment_group", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateEnvironmentGroup = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: [UPDATE_ENVIRONMENT_GROUP_QUERY_KEY],
    mutationFn: updateEnvironmentGroup,
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
