import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeAppOutput } from "@repo/moss-app";
import { useQuery } from "@tanstack/react-query";

export const USE_DESCRIBE_APP_QUERY_KEY = "describeApp";

const describeAppFn = async (): Promise<DescribeAppOutput> => {
  const result = await invokeTauriIpc<DescribeAppOutput>("describe_app");

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDescribeApp = () => {
  return useQuery<DescribeAppOutput, Error>({
    queryKey: [USE_DESCRIBE_APP_QUERY_KEY],
    queryFn: describeAppFn,
  });
};
