import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeAppStateOutput } from "@repo/moss-state";
import { useQuery } from "@tanstack/react-query";

export const USE_DESCRIBE_APP_STATE_QUERY_KEY = "describeAppState";

const describeAppStateFn = async (): Promise<DescribeAppStateOutput> => {
  const result = await invokeTauriIpc<DescribeAppStateOutput>("describe_app_state");

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDescribeAppState = () => {
  return useQuery<DescribeAppStateOutput, Error>({
    queryKey: [USE_DESCRIBE_APP_STATE_QUERY_KEY],
    queryFn: describeAppStateFn,
  });
};
