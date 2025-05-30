import { invokeTauriIpc } from "@/lib/backend/tauri";
import { CreateWorkspaceInput, CreateWorkspaceOutput } from "@repo/moss-workbench";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_CREATE_WORKSPACE_MUTATION_KEY = "createWorkspace";

const createWorkspaceFn = async (input: CreateWorkspaceInput): Promise<CreateWorkspaceOutput> => {
  const result = await invokeTauriIpc<CreateWorkspaceOutput>("create_workspace", {
    input: input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateWorkspace = () => {
  const queryClient = useQueryClient();
  return useMutation<CreateWorkspaceOutput, Error, CreateWorkspaceInput>({
    mutationKey: [USE_CREATE_WORKSPACE_MUTATION_KEY],
    mutationFn: createWorkspaceFn,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_LIST_WORKSPACES_QUERY_KEY] });
    },
  });
};
