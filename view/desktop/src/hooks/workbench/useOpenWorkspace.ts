import { invokeTauriIpc } from "@/lib/backend/tauri";
import { OpenWorkspaceInput, OpenWorkspaceOutput } from "@repo/moss-workbench";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "../appState/useDescribeAppState";
import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_OPEN_WORKSPACE_QUERY_KEY = "openWorkspace";

const openWorkspaceFn = async (workspaceId: string): Promise<OpenWorkspaceOutput> => {
  const result = await invokeTauriIpc<OpenWorkspaceOutput>("open_workspace", {
    input: {
      id: workspaceId,
    } as OpenWorkspaceInput,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useOpenWorkspace = () => {
  const queryClient = useQueryClient();
  return useMutation<OpenWorkspaceOutput, Error, string>({
    mutationKey: [USE_OPEN_WORKSPACE_QUERY_KEY],
    mutationFn: openWorkspaceFn,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_LIST_WORKSPACES_QUERY_KEY] });
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_APP_STATE_QUERY_KEY] });
    },
  });
};
