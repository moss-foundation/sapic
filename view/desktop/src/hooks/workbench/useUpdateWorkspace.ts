import { workspaceService } from "@/lib/services/workbench/workspaceService";
import { ListWorkspacesOutput, MainWindow_UpdateWorkspaceInput } from "@repo/ipc";
import { DescribeAppOutput } from "@repo/window";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_QUERY_KEY } from "../app/useDescribeApp";
import { useActiveWorkspace } from "../workspace";
import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_UPDATE_WORKSPACE_MUTATION_KEY = "updateWorkspace";

const updateWorkspaceFn = async (input: MainWindow_UpdateWorkspaceInput): Promise<void> => {
  const result = await workspaceService.updateWorkspace(input);

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateWorkspace = () => {
  const { activeWorkspace } = useActiveWorkspace();

  const queryClient = useQueryClient();
  return useMutation<void, Error, MainWindow_UpdateWorkspaceInput>({
    mutationKey: [USE_UPDATE_WORKSPACE_MUTATION_KEY],
    mutationFn: updateWorkspaceFn,
    onSuccess: (_, variables) => {
      queryClient.setQueryData<ListWorkspacesOutput>([USE_LIST_WORKSPACES_QUERY_KEY], (old) => {
        if (!old) return old;
        return old.map((workspace) => {
          if (workspace.id === activeWorkspace?.id) {
            return {
              ...workspace,
              name: variables.name ?? workspace.name,
            };
          }
          return workspace;
        });
      });

      queryClient.setQueryData<DescribeAppOutput>([USE_DESCRIBE_APP_QUERY_KEY], (old) => {
        if (!old || !old.workspace) return old;

        return {
          ...old,
          workspace: {
            ...old.workspace,
            name: variables.name ?? old.workspace.name,
          },
        };
      });
    },
  });
};
