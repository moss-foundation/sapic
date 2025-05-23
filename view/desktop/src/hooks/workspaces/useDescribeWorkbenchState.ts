import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useQuery } from "@tanstack/react-query";

export const USE_DESCRIBE_WORKBENCH_STATE_QUERY_KEY = "describeWorkbenchState";

interface WorkbenchState {
  currentWorkspace: string | null;
  recentWorkspaces: string[];
  availableWorkspaces: string[];
}

const describeWorkbenchStateFn = async (): Promise<WorkbenchState> => {
  const result = await invokeTauriIpc<WorkbenchState>("describe_workbench_state");

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDescribeWorkbenchState = () => {
  return useQuery<WorkbenchState, Error>({
    queryKey: [USE_DESCRIBE_WORKBENCH_STATE_QUERY_KEY],
    queryFn: describeWorkbenchStateFn,
  });
};
