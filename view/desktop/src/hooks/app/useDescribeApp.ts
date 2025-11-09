import { AppService } from "@/lib/services";
import { useQuery } from "@tanstack/react-query";

export const USE_DESCRIBE_APP_QUERY_KEY = "describeApp";

const describeAppFn = async () => {
  const result = await AppService.describeApp();

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDescribeApp = () => {
  return useQuery({
    queryKey: [USE_DESCRIBE_APP_QUERY_KEY],
    queryFn: describeAppFn,
  });
};
