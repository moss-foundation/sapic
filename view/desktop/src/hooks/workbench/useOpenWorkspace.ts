import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { DescribeWorkspaceOutput } from "@repo/moss-workspace";
import { DescribeAppOutput, OpenWorkspaceInput, OpenWorkspaceOutput } from "@repo/window";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_QUERY_KEY } from "../app/useDescribeApp";
import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, useStreamedProjectsWithResources } from "../project";
import { USE_STREAM_PROJECTS_QUERY_KEY } from "../project/useStreamProjects";
import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "../workspace/environment";
import { USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY } from "../workspace/useDescribeWorkspaceState";
import { useListWorkspaces } from "./useListWorkspaces";

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

  const { data: workspaces } = useListWorkspaces();
  const { data: projectsWithResources } = useStreamedProjectsWithResources();
  const { api } = useTabbedPaneStore();

  return useMutation<OpenWorkspaceOutput, Error, string>({
    mutationKey: [USE_OPEN_WORKSPACE_QUERY_KEY],
    mutationFn: openWorkspaceFn,
    onSuccess: (_, workspaceId) => {
      queryClient.removeQueries({
        queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY],
        exact: false,
      });

      queryClient.setQueryData([USE_DESCRIBE_APP_QUERY_KEY], (oldData: DescribeAppOutput | undefined) => {
        return {
          ...oldData,
          workspace: {
            id: workspaceId,
            name: workspaces?.find((workspace) => workspace.id === workspaceId)?.name || "",
            lastOpenedAt: undefined,
          },
        };
      });

      // Pre-fetch the new workspace state to ensure it's ready
      queryClient.prefetchQuery({
        queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY, workspaceId],
        queryFn: async (): Promise<DescribeWorkspaceOutput> => {
          // Small delay to ensure backend workspace switch is complete
          await new Promise((resolve) => setTimeout(resolve, 50));
          const result = await invokeTauriIpc<DescribeWorkspaceOutput>("describe_workspace");
          if (result.status === "error") {
            throw new Error(String(result.error));
          }
          return result.data;
        },
      });

      // Only invalidate workspace-specific data
      queryClient.removeQueries({ queryKey: [USE_STREAM_PROJECTS_QUERY_KEY] });
      queryClient.removeQueries({ queryKey: [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY] });
      queryClient.removeQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });

      // Remove panels that contain projects or resources that didn't come in streamed projects or resources for the new workspace
      projectsWithResources?.forEach((project) => {
        const projectPanelToRemove = api?.getPanel(project.id);

        if (projectPanelToRemove) {
          api?.removePanel(projectPanelToRemove);
        }

        project.resources.forEach((resource) => {
          const resourcePanelToRemove = api?.getPanel(resource.id);
          if (resourcePanelToRemove) api?.removePanel(resourcePanelToRemove);
        });
      });
    },
  });
};
