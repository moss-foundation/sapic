import { invokeTauriIpc } from "@/lib/backend/tauri";
import { OpenWorkspaceInput, OpenWorkspaceOutput } from "@repo/moss-app";
import { DescribeStateOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "../appState/useDescribeAppState";
import { USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY } from "../workspace/useDescribeWorkspaceState";
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
    onSuccess: (_, workspaceId) => {
      // Remove ALL cached workspace state queries to prevent stale data
      queryClient.removeQueries({
        queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY],
        exact: false,
      });

      // Invalidate other related queries
      queryClient.invalidateQueries({ queryKey: [USE_LIST_WORKSPACES_QUERY_KEY] });
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_APP_STATE_QUERY_KEY] });

      // Pre-fetch the new workspace state to ensure it's ready
      queryClient.prefetchQuery({
        queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY, workspaceId],
        queryFn: async (): Promise<DescribeStateOutput> => {
          // Small delay to ensure backend workspace switch is complete
          await new Promise((resolve) => setTimeout(resolve, 50));
          const result = await invokeTauriIpc<DescribeStateOutput>("describe_workspace_state");
          if (result.status === "error") {
            throw new Error(String(result.error));
          }
          return result.data;
        },
      });
    },
  });
};
