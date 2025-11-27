import { workspaceService } from "@/domains/workspace/workspaceService";
import { DeleteWorkspaceInput, DeleteWorkspaceOutput, ListWorkspacesOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useRemoveLayout } from "../../../hooks/workbench/layout";
import { useCloseWorkspace } from "../../../hooks/workbench/useCloseWorkspace";
import { useActiveWorkspace } from "../../../hooks/workspace/derived/useActiveWorkspace";
import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_DELETE_WORKSPACE_MUTATION_KEY = "deleteWorkspace";

const deleteWorkspaceFn = async (input: DeleteWorkspaceInput): Promise<DeleteWorkspaceOutput> => {
  return await workspaceService.delete(input);
};

export const useDeleteWorkspace = () => {
  const queryClient = useQueryClient();

  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();
  const { mutateAsync: closeWorkspace } = useCloseWorkspace();
  const { mutateAsync: removeLayout } = useRemoveLayout();

  return useMutation<DeleteWorkspaceOutput, Error, DeleteWorkspaceInput>({
    mutationKey: [USE_DELETE_WORKSPACE_MUTATION_KEY],
    mutationFn: async (input) => {
      if (hasActiveWorkspace && activeWorkspaceId === input.id) {
        try {
          await closeWorkspace(activeWorkspaceId);
        } catch (error) {
          throw new Error(`Failed to close workspace: ${error}`);
        }
      }

      return deleteWorkspaceFn(input);
    },
    onSuccess: async (_, variables) => {
      queryClient.setQueryData([USE_LIST_WORKSPACES_QUERY_KEY], (oldData: ListWorkspacesOutput) => {
        return oldData.filter((workspace) => workspace.id !== variables.id);
      });

      await removeLayout({ workspaceId: variables.id });
    },
  });
};
