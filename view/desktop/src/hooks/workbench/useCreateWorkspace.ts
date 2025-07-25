import { invokeTauriIpc } from "@/lib/backend/tauri";
import {
  CreateWorkspaceInput,
  CreateWorkspaceOutput,
  DescribeAppStateOutput,
  ListWorkspacesOutput,
  WorkspaceInfo,
} from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "../appState/useDescribeAppState";
import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "../collection";
import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "../collection/useStreamedCollections";
import { USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY } from "../workspace/useDescribeWorkspaceState";
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
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({ queryKey: [USE_LIST_WORKSPACES_QUERY_KEY] });

      // If workspace was opened automatically by backend, update caches accordingly
      if (variables.openOnCreation && data.active) {
        // Clear workspace state queries since we're switching workspaces
        queryClient.removeQueries({
          queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY],
          exact: false,
        });

        queryClient.setQueryData([USE_DESCRIBE_APP_STATE_QUERY_KEY], (oldData: DescribeAppStateOutput | undefined) => {
          if (oldData) {
            return {
              ...oldData,
              prevWorkspaceId: data.id,
            };
          }
          return oldData;
        });

        queryClient.setQueryData([USE_LIST_WORKSPACES_QUERY_KEY], (oldData: ListWorkspacesOutput | undefined) => {
          if (Array.isArray(oldData)) {
            return oldData.map((workspace: WorkspaceInfo) => ({
              ...workspace,
              active: workspace.id === data.id,
            }));
          }
          return oldData;
        });

        queryClient.invalidateQueries({ queryKey: [USE_STREAMED_COLLECTIONS_QUERY_KEY], exact: true });
        queryClient.invalidateQueries({ queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY], exact: true });
      }
    },
  });
};
