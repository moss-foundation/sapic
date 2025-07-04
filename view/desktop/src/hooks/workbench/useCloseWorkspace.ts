import { invokeTauriIpc } from "@/lib/backend/tauri";
import { CloseWorkspaceInput, CloseWorkspaceOutput } from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "../appState/useDescribeAppState";
import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, USE_STREAMED_COLLECTIONS_QUERY_KEY } from "../collection";
import { USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY } from "../workspace/useDescribeWorkspaceState";
import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_CLOSE_WORKSPACE_QUERY_KEY = "closeWorkspace";

const closeWorkspaceFn = async (workspaceId: string): Promise<CloseWorkspaceOutput> => {
  const result = await invokeTauriIpc<CloseWorkspaceOutput>("close_workspace", {
    input: {
      id: workspaceId,
    } as CloseWorkspaceInput,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCloseWorkspace = () => {
  const queryClient = useQueryClient();
  return useMutation<CloseWorkspaceOutput, Error, string>({
    mutationKey: [USE_CLOSE_WORKSPACE_QUERY_KEY],
    mutationFn: closeWorkspaceFn,
    onSuccess: () => {
      // Remove ALL cached workspace state queries since no workspace is active
      queryClient.removeQueries({
        queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY],
        exact: false,
      });
      queryClient.removeQueries({ queryKey: [USE_STREAMED_COLLECTIONS_QUERY_KEY], exact: true });
      queryClient.removeQueries({ queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY], exact: true });

      // Invalidate other related queries
      queryClient.invalidateQueries({ queryKey: [USE_LIST_WORKSPACES_QUERY_KEY] });
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_APP_STATE_QUERY_KEY] });

      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_COLLECTIONS_QUERY_KEY], exact: true });
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY], exact: true });
    },
  });
};
