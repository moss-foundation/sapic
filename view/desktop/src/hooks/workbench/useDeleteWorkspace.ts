import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DeleteWorkspaceInput, DeleteWorkspaceOutput } from "@repo/moss-workbench";
import { useMutation, useQueryClient } from "@tanstack/react-query";

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
  return useMutation<DeleteWorkspaceOutput, Error, DeleteWorkspaceInput>({
    mutationKey: [USE_DELETE_WORKSPACE_MUTATION_KEY],
    mutationFn: deleteWorkspaceFn,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_LIST_WORKSPACES_QUERY_KEY] });
    },
  });
}; 