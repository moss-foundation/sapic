import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeAppStateOutput } from "@repo/moss-state";
import { useQuery } from "@tanstack/react-query";

const describeAppStateFn = async (): Promise<DescribeAppStateOutput> => {
  const result = await invokeTauriIpc<DescribeAppStateOutput>("describe_app_state");
  if (result.status === "error") {
    throw new Error(String(result.error));
  }
  return result.data;
};

export const useDescribeAppState = () => {
  return useQuery<DescribeAppStateOutput, Error>({
    queryKey: ["describeAppState"],
    queryFn: describeAppStateFn,
  });
};
