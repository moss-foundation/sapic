import { invokeTauriIpc } from "@/lib/backend/tauri";
import { UpdateWorkspaceInput } from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_UPDATE_WORKSPACE_MUTATION_KEY = "updateWorkspace";

const updateWorkspaceFn = async (input: UpdateWorkspaceInput): Promise<void> => {
  const result = await invokeTauriIpc<void>("update_workspace", {
    input: input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateWorkspace = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, UpdateWorkspaceInput>({
    mutationKey: [USE_UPDATE_WORKSPACE_MUTATION_KEY],
    mutationFn: updateWorkspaceFn,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_LIST_WORKSPACES_QUERY_KEY] });
    },
  });
};
