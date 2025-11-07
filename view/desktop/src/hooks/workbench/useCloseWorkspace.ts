import { workspaceService } from "@/lib/services/workbench/workspaceService";
import { CloseWorkspaceOutput, DescribeAppOutput } from "@repo/window";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, USE_STREAM_PROJECTS_QUERY_KEY } from "..";
import { USE_DESCRIBE_APP_QUERY_KEY } from "../app/useDescribeApp";
import { USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "../workspace/environment";

export const USE_CLOSE_WORKSPACE_QUERY_KEY = "closeWorkspace";

const closeWorkspaceFn = async (workspaceId: string): Promise<CloseWorkspaceOutput> => {
  const result = await workspaceService.closeWorkspace({
    id: workspaceId,
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
      queryClient.setQueryData([USE_DESCRIBE_APP_QUERY_KEY], (old: DescribeAppOutput) => {
        return { ...old, workspace: null };
      });

      // Remove ALL cached workspace state queries since no workspace is active
      queryClient.removeQueries({ queryKey: [USE_STREAM_PROJECTS_QUERY_KEY] });
      queryClient.removeQueries({ queryKey: [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY] });
      queryClient.removeQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
    },
  });
};
