import { invokeTauriIpc } from "@/lib/backend/tauri";
import { CreateWorkspaceInput, CreateWorkspaceOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_WORKSPACE_QUERY_KEY } from "./useGetWorkspaces";

export const USE_CREATE_WORKSPACE_MUTATION_KEY = "createWorkspace";

const createWorkspaceFn = async (input: CreateWorkspaceInput): Promise<CreateWorkspaceOutput> => {
  const result = await invokeTauriIpc<CreateWorkspaceOutput, CreateWorkspaceInput>("create_workspace", {
    input,
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
      console.log("About to invalidate queries USE_GET_WORKSPACE_QUERY_KEY", USE_GET_WORKSPACE_QUERY_KEY);
      queryClient.invalidateQueries({ queryKey: [USE_GET_WORKSPACE_QUERY_KEY] });
    },
  });
};
