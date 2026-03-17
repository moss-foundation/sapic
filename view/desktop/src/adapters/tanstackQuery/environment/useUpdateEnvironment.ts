import { environmentService } from "@/domains/environment/environmentService";
import {
  ListEnvironmentItem,
  ListWorkspaceEnvironmentsOutput,
  UpdateEnvironmentInput,
  UpdateEnvironmentOutput,
} from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY } from "./useListWorkspaceEnvironments";

const UPDATE_ENVIRONMENT_QUERY_KEY = "updateEnvironment";

export const useUpdateEnvironment = () => {
  const queryClient = useQueryClient();
  return useMutation<UpdateEnvironmentOutput, Error, UpdateEnvironmentInput>({
    mutationKey: [UPDATE_ENVIRONMENT_QUERY_KEY],
    mutationFn: (input) => environmentService.updateEnvironment(input),
    onSuccess: (_, variables) => {
      queryClient.setQueryData(
        [USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY],
        (old: ListWorkspaceEnvironmentsOutput | undefined) => {
          if (!old) return old;

          const newItems: ListEnvironmentItem[] = old.items.map((environment) => {
            if (environment.id === variables.id) {
              return {
                ...environment,
                name: variables.name ?? environment.name,
                color:
                  variables.color && typeof variables.color === "object" && "UPDATE" in variables.color
                    ? variables.color.UPDATE
                    : variables.color === "REMOVE"
                      ? undefined
                      : environment.color,
                totalVariables: environment.totalVariables + variables.varsToAdd.length - variables.varsToDelete.length,
                isActive: environment.isActive,
              } satisfies ListEnvironmentItem;
            }

            return environment;
          });

          return {
            items: newItems,
          } satisfies ListWorkspaceEnvironmentsOutput;
        }
      );
    },
  });
};
