import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { environmentService } from "@/domains/environment/environmentService";
import { BatchUpdateEnvironmentInput, BatchUpdateEnvironmentOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY } from "./useListWorkspaceEnvironments";

const BATCH_UPDATE_ENVIRONMENT_QUERY_KEY = "batchUpdateEnvironment";

export const useBatchUpdateEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation<BatchUpdateEnvironmentOutput, Error, BatchUpdateEnvironmentInput>({
    mutationKey: [BATCH_UPDATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: (input) => environmentService.updateEnvironment(input),
    onSuccess: (_, variables) => {
      queryClient.setQueryData([USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY], (old: EnvironmentSummary[]) => {
        return old.map((oldEnv) => {
          const updatedEnv = variables.items.find((updatedEnv) => updatedEnv.id === oldEnv.id);
          if (updatedEnv) {
            return {
              ...oldEnv,
              name: updatedEnv.name ?? oldEnv.name,
              color:
                updatedEnv.color && typeof updatedEnv.color === "object" && "UPDATE" in updatedEnv.color
                  ? updatedEnv.color.UPDATE
                  : updatedEnv.color === "REMOVE"
                    ? null
                    : oldEnv.color,
              totalVariables: updatedEnv.varsToAdd.length - updatedEnv.varsToDelete.length,
            } satisfies EnvironmentSummary;
          }
          return oldEnv;
        });
      });
    },
  });
};
