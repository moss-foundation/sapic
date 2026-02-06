import { environmentService } from "@/domains/environment/environmentService";
import { StreamEnvironmentsEvent, UpdateEnvironmentInput, UpdateEnvironmentOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "./useStreamEnvironments";
import { USE_STREAMED_PROJECT_ENVIRONMENTS_QUERY_KEY } from "./useStreamProjectEnvironments";

const UPDATE_ENVIRONMENT_QUERY_KEY = "updateEnvironment";

export const useUpdateEnvironment = () => {
  const queryClient = useQueryClient();

  return useMutation<UpdateEnvironmentOutput, Error, UpdateEnvironmentInput>({
    mutationKey: [UPDATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: (input) => environmentService.updateEnvironment(input),
    onSuccess: (data, variables) => {
      const isProjectEnvironment = variables.projectId !== undefined && variables.projectId !== null;
      const isWorkspaceEnvironment = !variables.projectId;

      if (isWorkspaceEnvironment) {
        queryClient.setQueryData([USE_STREAMED_ENVIRONMENTS_QUERY_KEY], (old: StreamEnvironmentsEvent[]) => {
          return old.map((environment) =>
            environment.id === variables.id
              ? {
                  ...environment,
                  ...data,
                  ...variables,
                }
              : environment
          );
        });
      }

      if (isProjectEnvironment) {
        queryClient.setQueryData(
          [USE_STREAMED_PROJECT_ENVIRONMENTS_QUERY_KEY, variables.projectId],
          (old: StreamEnvironmentsEvent[]) => {
            return old.map((oldEnv) => {
              if (oldEnv.projectId !== variables.projectId) return oldEnv;

              const newEnv: StreamEnvironmentsEvent = {
                ...oldEnv,
                ...data,
              };

              if (variables.name) newEnv.name = variables.name;
              if (typeof variables.color === "object" && "UPDATE" in variables.color)
                newEnv.color = variables.color.UPDATE;
              if (variables.color === "REMOVE") newEnv.color = undefined;
              if (variables.order) newEnv.order = variables.order;
              if (variables.varsToAdd.length > 0) newEnv.totalVariables += variables.varsToAdd.length;
              if (variables.varsToDelete.length > 0) newEnv.totalVariables -= variables.varsToDelete.length;

              return newEnv;
            });
          }
        );
      }
    },
  });
};
