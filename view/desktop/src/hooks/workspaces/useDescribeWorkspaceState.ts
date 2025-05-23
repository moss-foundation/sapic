import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeStateOutput } from "@repo/moss-workspace";
import { useQuery } from "@tanstack/react-query";

export const USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY = "describeWorkspaceState";

const describeWorkspaceStateFn = async (workspaceId: string | null): Promise<DescribeStateOutput | null> => {
  if (!workspaceId) return null;

  const result = await invokeTauriIpc<DescribeStateOutput>("describe_workspace_state", { workspaceId });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDescribeWorkspaceState = (workspaceId: string | null) => {
  return useQuery<DescribeStateOutput | null, Error>({
    queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY, workspaceId],
    queryFn: () => describeWorkspaceStateFn(workspaceId),
    enabled: !!workspaceId,
  });
};
