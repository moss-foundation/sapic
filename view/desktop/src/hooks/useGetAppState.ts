import { getState as describeAppState } from "@/api/appearance";
import { DescribeAppStateOutput } from "@repo/moss-state";
import { useQuery } from "@tanstack/react-query";

export const useGetAppState = () => {
  return useQuery<DescribeAppStateOutput, Error>({
    queryKey: ["getState"],
    queryFn: describeAppState,
  });
};
