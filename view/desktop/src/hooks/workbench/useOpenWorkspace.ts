import { workspaceService } from "@/lib/services/workbench/workspaceService";
import { DescribeAppOutput, OpenWorkspaceOutput } from "@repo/moss-app";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_QUERY_KEY } from "../app/useDescribeApp";
import { useListWorkspaces } from "./useListWorkspaces";

export const USE_OPEN_WORKSPACE_QUERY_KEY = "openWorkspace";

const openWorkspaceFn = async (workspaceId: string): Promise<OpenWorkspaceOutput> => {
  const result = await workspaceService.openWorkspace({
    id: workspaceId,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useOpenWorkspace = () => {
  const queryClient = useQueryClient();

  const { data: workspaces } = useListWorkspaces();

  return useMutation<OpenWorkspaceOutput, Error, string>({
    mutationKey: [USE_OPEN_WORKSPACE_QUERY_KEY],
    mutationFn: openWorkspaceFn,
    onSuccess: (_, workspaceId) => {
      const openedWorkspace = workspaces?.find((workspace) => workspace.id === workspaceId);

      if (!openedWorkspace) return;

      queryClient.setQueryData([USE_DESCRIBE_APP_QUERY_KEY], (oldData: DescribeAppOutput | undefined) => {
        return {
          ...oldData,
          workspace: openedWorkspace,
        };
      });
    },
  });
};
