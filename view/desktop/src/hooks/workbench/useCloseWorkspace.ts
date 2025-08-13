import { invokeTauriIpc } from "@/lib/backend/tauri";
import { CloseWorkspaceInput, CloseWorkspaceOutput, ListWorkspacesOutput } from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "..";
import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "../appState/useDescribeAppState";
import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "../collection";
import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "../environment";
import { USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY } from "../workspace";
import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_CLOSE_WORKSPACE_QUERY_KEY = "closeWorkspace";

const closeWorkspaceFn = async (workspaceId: string): Promise<CloseWorkspaceOutput> => {
  console.log("onSuccess");
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
      queryClient.setQueryData([USE_LIST_WORKSPACES_QUERY_KEY], (old: ListWorkspacesOutput) => {
        return old.map((workspace) => ({ ...workspace, active: false }));
      });
      queryClient.invalidateQueries({ queryKey: [USE_LIST_WORKSPACES_QUERY_KEY] });

      // Invalidate other related queries
      queryClient.invalidateQueries({ queryKey: [USE_DESCRIBE_APP_STATE_QUERY_KEY] });

      // Remove ALL cached workspace state queries since no workspace is active
      queryClient.removeQueries({ queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY] });
      queryClient.removeQueries({ queryKey: [USE_STREAMED_COLLECTIONS_QUERY_KEY] });
      queryClient.removeQueries({ queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY] });
      queryClient.removeQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
    },
  });
};
