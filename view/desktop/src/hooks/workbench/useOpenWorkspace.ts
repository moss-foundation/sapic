import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import {
  DescribeAppOutput,
  DescribeAppStateOutput,
  ListWorkspacesOutput,
  OpenWorkspaceInput,
  OpenWorkspaceOutput,
  WorkspaceInfo,
} from "@repo/moss-app";
import { DescribeStateOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_STATE_QUERY_KEY } from "../app/useDescribeAppState";
import { USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, useStreamedProjectsWithEntries } from "../project";
import { USE_STREAM_PROJECTS_QUERY_KEY } from "../project/useStreamProjects";
import { USE_DESCRIBE_APP_QUERY_KEY } from "../useDescribeApp";
import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "../workspace/environment";
import { USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY } from "../workspace/useDescribeWorkspaceState";
import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_OPEN_WORKSPACE_QUERY_KEY = "openWorkspace";

const openWorkspaceFn = async (workspaceId: string): Promise<OpenWorkspaceOutput> => {
  const result = await invokeTauriIpc<OpenWorkspaceOutput>("open_workspace", {
    input: {
      id: workspaceId,
    } as OpenWorkspaceInput,
  });

  console.log({
    openWorkspaceResult: result,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useOpenWorkspace = () => {
  const queryClient = useQueryClient();

  const { data: projectsWithEntries } = useStreamedProjectsWithEntries();
  const { api } = useTabbedPaneStore();

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
            prevWorkspaceId: workspaceId,
          };
        }
        return oldData;
      });

      queryClient.setQueryData([USE_DESCRIBE_APP_QUERY_KEY], (oldData: DescribeAppOutput | undefined) => {
        if (oldData) {
          return {
            ...oldData,
            workspace: workspaceId,
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
      queryClient.removeQueries({ queryKey: [USE_STREAM_PROJECTS_QUERY_KEY] });
      queryClient.removeQueries({ queryKey: [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY] });
      queryClient.removeQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });

      // Remove panels that contain projects or entries that didn't come in streamed projects or entries for the new workspace
      projectsWithEntries?.forEach((project) => {
        const projectPanelToRemove = api?.getPanel(project.id);

        if (projectPanelToRemove) {
          api?.removePanel(projectPanelToRemove);
        }

        project.entries.forEach((entry) => {
          const entryPanelToRemove = api?.getPanel(entry.id);
          if (entryPanelToRemove) api?.removePanel(entryPanelToRemove);
        });
      });
    },
  });
};
