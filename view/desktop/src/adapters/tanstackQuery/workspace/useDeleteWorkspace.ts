import { workspaceService } from "@/domains/workspace/workspaceService";
import { useRemoveLayout } from "@/workbench/adapters";
import { DeleteWorkspaceInput, DeleteWorkspaceOutput, ListWorkspacesOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_DELETE_WORKSPACE_MUTATION_KEY = "deleteWorkspace";

export const useDeleteWorkspace = () => {
  const queryClient = useQueryClient();

  const { mutateAsync: removeLayout } = useRemoveLayout();

  return useMutation<DeleteWorkspaceOutput, Error, DeleteWorkspaceInput>({
    mutationKey: [USE_DELETE_WORKSPACE_MUTATION_KEY],
    mutationFn: workspaceService.delete,
    onSuccess: async (_, variables) => {
      queryClient.setQueryData([USE_LIST_WORKSPACES_QUERY_KEY], (oldData: ListWorkspacesOutput) => {
        return oldData.filter((workspace) => workspace.id !== variables.id);
      });

      await removeLayout(variables.id);
    },
  });
};
