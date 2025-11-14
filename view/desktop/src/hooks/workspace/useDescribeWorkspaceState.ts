import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { DescribeWorkspaceOutput } from "@repo/moss-workspace";
import { useQuery } from "@tanstack/react-query";

import { useActiveWorkspace } from "./derived/useActiveWorkspace";

/**
 * @deprecated we now use shared storage to get layouts
 */
export const USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY = "describeWorkspaceState";

const describeWorkspaceStateFn = async (): Promise<DescribeWorkspaceOutput> => {
  const result = await invokeTauriIpc<DescribeWorkspaceOutput>("describe_workspace");

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

interface UseDescribeWorkspaceStateOptions {
  enabled?: boolean;
}

/**
 * @deprecated we now use shared storage to get layouts
 */
export const useDescribeWorkspaceState = ({ enabled = true }: UseDescribeWorkspaceStateOptions = {}) => {
  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();

  return useQuery<DescribeWorkspaceOutput, Error>({
    queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY, activeWorkspaceId],
    queryFn: describeWorkspaceStateFn,
    enabled: enabled && hasActiveWorkspace,
  });
};
