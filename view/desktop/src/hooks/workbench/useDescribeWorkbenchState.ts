/* FIXME: This hook is currently not used */

import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeWorkbenchStateOutput } from "@repo/moss-app";
import { useQuery } from "@tanstack/react-query";

export const USE_DESCRIBE_WORKBENCH_STATE_QUERY_KEY = "describeWorkbenchState";

const describeWorkbenchStateFn = async (): Promise<DescribeWorkbenchStateOutput> => {
  const result = await invokeTauriIpc<DescribeWorkbenchStateOutput>("describe_workbench_state");

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDescribeWorkbenchState = () => {
  return useQuery<DescribeWorkbenchStateOutput, Error>({
    queryKey: [USE_DESCRIBE_WORKBENCH_STATE_QUERY_KEY],
    queryFn: describeWorkbenchStateFn,
  });
};
