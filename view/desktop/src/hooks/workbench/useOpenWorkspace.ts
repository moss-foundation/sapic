import { invokeTauriIpc } from "@/lib/backend/tauri";
import {
  OpenWorkspaceInput,
  OpenWorkspaceOutput,
  DescribeAppStateOutput,
  ListWorkspacesOutput,
  WorkspaceInfo,
} from "@repo/moss-app";
import { DescribeStateOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "../appState/useDescribeAppState";
import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "../collection";
import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "../collection/useStreamedCollections";
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
      queryClient.removeQueries({
        queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY],
        exact: false,
      });

      queryClient.setQueryData([USE_DESCRIBE_APP_STATE_QUERY_KEY], (oldData: DescribeAppStateOutput | undefined) => {
        if (oldData) {
          return {
            ...oldData,
            lastWorkspace: workspaceId,
          };
        }
        return oldData;
      });

      queryClient.setQueryData([USE_LIST_WORKSPACES_QUERY_KEY], (oldData: ListWorkspacesOutput | undefined) => {
        if (Array.isArray(oldData)) {
          return oldData.map((workspace: WorkspaceInfo) => ({
            ...workspace,
            active: workspace.id === workspaceId,
          }));
        }
        return oldData;
      });

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

      // Only invalidate workspace-specific data
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_COLLECTIONS_QUERY_KEY], exact: true });
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY], exact: true });
    },
  });
};
