import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeStateOutput } from "@repo/moss-workspace";
import { useQuery } from "@tanstack/react-query";

import { useActiveWorkspace } from "./useActiveWorkspace";

export const USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY = "describeWorkspaceState";

const describeWorkspaceStateFn = async (): Promise<DescribeStateOutput> => {
  const result = await invokeTauriIpc<DescribeStateOutput>("describe_workspace_state");

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

interface UseDescribeWorkspaceStateOptions {
  enabled?: boolean;
}

export const useDescribeWorkspaceState = ({ enabled = true }: UseDescribeWorkspaceStateOptions = {}) => {
  const workspace = useActiveWorkspace();
  const workspaceId = workspace?.id || null;

  return useQuery<DescribeStateOutput, Error>({
    queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY, workspaceId],
    queryFn: describeWorkspaceStateFn,
    enabled: enabled && !!workspaceId,
  });
};
