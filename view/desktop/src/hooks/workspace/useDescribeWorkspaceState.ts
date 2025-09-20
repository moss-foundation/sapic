import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeWorkspaceOutput } from "@repo/moss-workspace";
import { useQuery } from "@tanstack/react-query";

import { useActiveWorkspace } from "./derived/useActiveWorkspace";

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

export const useDescribeWorkspaceState = ({ enabled = true }: UseDescribeWorkspaceStateOptions = {}) => {
  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();

  return useQuery<DescribeWorkspaceOutput, Error>({
    queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY, activeWorkspaceId],
    queryFn: describeWorkspaceStateFn,
    enabled: enabled && hasActiveWorkspace,
  });
};
