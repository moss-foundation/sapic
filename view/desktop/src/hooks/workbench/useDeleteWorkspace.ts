import { workspaceService } from "@/lib/services/workbench/workspaceService";
import { DeleteWorkspaceInput, DeleteWorkspaceOutput, ListWorkspacesOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useActiveWorkspace } from "../workspace/derived/useActiveWorkspace";
import { useRemoveLayout } from "./layout";
import { useCloseWorkspace } from "./useCloseWorkspace";
import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_DELETE_WORKSPACE_MUTATION_KEY = "deleteWorkspace";

const deleteWorkspaceFn = async (input: DeleteWorkspaceInput): Promise<DeleteWorkspaceOutput> => {
  const result = await workspaceService.deleteWorkspace(input);

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteWorkspace = () => {
  const queryClient = useQueryClient();

  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();
  const { mutateAsync: closeWorkspace } = useCloseWorkspace();

  const { mutateAsync: removeLayout } = useRemoveLayout();

  return useMutation<DeleteWorkspaceOutput, Error, DeleteWorkspaceInput>({
    mutationKey: [USE_DELETE_WORKSPACE_MUTATION_KEY],
    mutationFn: async (input: DeleteWorkspaceInput) => {
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
