import { mainWorkspaceService } from "@/main/services/mainWindowWorkspaceService";
import { ListWorkspacesOutput, MainWindow_UpdateWorkspaceInput, MainWindow_UpdateWorkspaceOutput } from "@repo/ipc";
import { DescribeAppOutput } from "@repo/window";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_WORKSPACES_QUERY_KEY } from "../../adapters/tanstackQuery/workspace/useListWorkspaces";
import { USE_DESCRIBE_APP_QUERY_KEY } from "../app/useDescribeApp";
import { useActiveWorkspace } from "../workspace";

export const USE_UPDATE_WORKSPACE_MUTATION_KEY = "updateWorkspace";

const updateWorkspaceFn = async (input: MainWindow_UpdateWorkspaceInput): Promise<MainWindow_UpdateWorkspaceOutput> => {
  return await mainWorkspaceService.update(input);
};

export const useUpdateWorkspace = () => {
  const { activeWorkspace } = useActiveWorkspace();

  const queryClient = useQueryClient();
  return useMutation<MainWindow_UpdateWorkspaceOutput, Error, MainWindow_UpdateWorkspaceInput>({
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
