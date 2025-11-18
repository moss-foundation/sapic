import { projectIpc } from "@/infra/ipc/project";
import { DescribeResourceOutput } from "@repo/moss-project";
import { useQuery, UseQueryOptions } from "@tanstack/react-query";

export const USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY = "describeProjectResource";

export interface UseDescribeProjectResourceProps {
  projectId: string;
  resourceId: string;
  options?: Omit<UseQueryOptions<DescribeResourceOutput, Error>, "queryKey" | "queryFn">;
}

export const useDescribeProjectResource = ({ projectId, resourceId, options }: UseDescribeProjectResourceProps) => {
  return useQuery<DescribeResourceOutput, Error>({
    queryKey: [USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY, projectId, resourceId],
    queryFn: () => projectIpc.describeProjectResource(projectId, resourceId),
    ...options,
  });
};
