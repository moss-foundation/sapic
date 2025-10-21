import { ProjectService } from "@/lib/services/projectService";
import { DescribeResourceOutput } from "@repo/moss-project";
import { useQuery, UseQueryOptions } from "@tanstack/react-query";

export const USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY = "describeProjectResource";

const describeProjectResourceFn = async ({ projectId, resourceId }: UseDescribeProjectResourceProps) => {
  const result = await ProjectService.describeProjectResource({ projectId, resourceId });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export interface UseDescribeProjectResourceProps {
  projectId: string;
  resourceId: string;
  options?: Omit<UseQueryOptions<DescribeResourceOutput, Error>, "queryKey" | "queryFn">;
}

export const useDescribeProjectResource = ({ projectId, resourceId, options }: UseDescribeProjectResourceProps) => {
  return useQuery({
    queryKey: [USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY, projectId, resourceId],
    queryFn: () => describeProjectResourceFn({ projectId, resourceId }),
    ...options,
  });
};
