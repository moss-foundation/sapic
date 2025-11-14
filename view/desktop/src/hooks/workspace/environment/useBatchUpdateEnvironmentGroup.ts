import { StreamEnvironmentsResult } from "@/hooks/workspace/environment/useStreamEnvironments";
import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { BatchUpdateEnvironmentGroupInput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "..";

const BATCH_UPDATE_ENVIRONMENT_GROUP_MUTATION_KEY = "batchUpdateEnvironmentGroup";

const batchUpdateEnvironmentGroup = async (input: BatchUpdateEnvironmentGroupInput) => {
  const result = await invokeTauriIpc("batch_update_environment_group", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useBatchUpdateEnvironmentGroup = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: [BATCH_UPDATE_ENVIRONMENT_GROUP_MUTATION_KEY],
    mutationFn: batchUpdateEnvironmentGroup,
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
