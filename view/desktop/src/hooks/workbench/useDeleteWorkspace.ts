import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DeleteWorkspaceInput, DeleteWorkspaceOutput } from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useActiveWorkspace } from "../workspace/useActiveWorkspace";
import { useCloseWorkspace } from "./useCloseWorkspace";
import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_DELETE_WORKSPACE_MUTATION_KEY = "deleteWorkspace";

const deleteWorkspaceFn = async (input: DeleteWorkspaceInput): Promise<DeleteWorkspaceOutput> => {
  const result = await invokeTauriIpc<DeleteWorkspaceOutput>("delete_workspace", {
    input: input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteWorkspace = () => {
  const queryClient = useQueryClient();
  const activeWorkspace = useActiveWorkspace();
  const { mutateAsync: closeWorkspace } = useCloseWorkspace();

  return useMutation<DeleteWorkspaceOutput, Error, DeleteWorkspaceInput>({
    mutationKey: [USE_DELETE_WORKSPACE_MUTATION_KEY],
    mutationFn: async (input: DeleteWorkspaceInput) => {
      if (activeWorkspace && activeWorkspace.id === input.id) {
        try {
          await closeWorkspace(activeWorkspace.id);
        } catch (error) {
          throw new Error(`Failed to close workspace: ${error}`);
        }
      }

      return deleteWorkspaceFn(input);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_LIST_WORKSPACES_QUERY_KEY] });
    },
  });
};
