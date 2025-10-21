import { ProjectService } from "@/lib/services/projectService";
import { DescribeResourceOutput } from "@repo/moss-project";
import { useQuery, UseQueryOptions } from "@tanstack/react-query";

export const USE_DESCRIBE_PROJECT_ENTRY_QUERY_KEY = "describeProjectEntry";

const describeProjectEntryFn = async ({ projectId, entryId }: UseDescribeProjectEntryProps) => {
  const result = await ProjectService.describeProjectEntry({ projectId, entryId });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export interface UseDescribeProjectEntryProps {
  projectId: string;
  entryId: string;
  options?: Omit<UseQueryOptions<DescribeResourceOutput, Error>, "queryKey" | "queryFn">;
}

export const useDescribeProjectEntry = ({ projectId, entryId, options }: UseDescribeProjectEntryProps) => {
  return useQuery({
    queryKey: [USE_DESCRIBE_PROJECT_ENTRY_QUERY_KEY, projectId, entryId],
    queryFn: () => describeProjectEntryFn({ projectId, entryId }),
    ...options,
  });
};
