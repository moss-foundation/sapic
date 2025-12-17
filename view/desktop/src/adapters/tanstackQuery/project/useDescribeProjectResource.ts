import { projectService } from "@/domains/project/projectService";
import { DescribeResourceOutput } from "@repo/moss-project";
import { keepPreviousData, useQuery, useQueryClient, UseQueryOptions } from "@tanstack/react-query";

export const USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY = "describeProjectResource";

export interface UseDescribeProjectResourceProps {
  projectId: string;
  resourceId: string;
  options?: Omit<UseQueryOptions<DescribeResourceOutput, Error>, "queryKey" | "queryFn">;
}

export const useDescribeProjectResource = ({ projectId, resourceId, options }: UseDescribeProjectResourceProps) => {
  const queryClient = useQueryClient();

  return useQuery<DescribeResourceOutput, Error>({
    queryKey: [USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY, projectId, resourceId],
    queryFn: async () => {
      const data = await projectService.describeProjectResource(projectId, resourceId);
      //TODO: this is a temporary solution to preserve the existing URL from cache
      //If backend returns "Hardcoded Value", preserve existing URL from cache
      if (data.url === "Hardcoded Value") {
        const cachedData = queryClient.getQueryData<DescribeResourceOutput>([
          USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY,
          projectId,
          resourceId,
        ]);

        // Preserve existing URL if it exists and is not "Hardcoded Value"
        if (cachedData?.url && cachedData.url !== "Hardcoded Value") {
          return {
            ...data,
            url: cachedData.url,
          };
        }

        // Otherwise, set URL to undefined
        return {
          ...data,
          url: undefined,
        };
      }

      return data;
    },
    placeholderData: keepPreviousData,
    enabled: !!projectId && !!resourceId,
    ...options,
  });
};
