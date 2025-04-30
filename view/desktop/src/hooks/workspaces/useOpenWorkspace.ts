import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useAllowDescribeWorkspaceStore } from "@/store/allowDescribeWorkspace";
import { OpenWorkspaceInput, OpenWorkspaceOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY } from "./useDescribeWorkspaceState";
import { USE_GET_WORKSPACE_QUERY_KEY } from "./useGetWorkspaces";

export const USE_OPEN_WORKSPACE_QUERY_KEY = "openWorkspace";

const openWorkspaceFn = async (name: string): Promise<OpenWorkspaceOutput> => {
  const result = await invokeTauriIpc<OpenWorkspaceOutput, OpenWorkspaceInput>("open_workspace", {
    input: { name },
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useOpenWorkspace = () => {
  const queryClient = useQueryClient();
  const { setAllow } = useAllowDescribeWorkspaceStore();

  return useMutation<OpenWorkspaceOutput, Error, string>({
    mutationKey: [USE_OPEN_WORKSPACE_QUERY_KEY],
    mutationFn: openWorkspaceFn,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_GET_WORKSPACE_QUERY_KEY] });
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY] });
      setAllow(true);
    },
  });
};
