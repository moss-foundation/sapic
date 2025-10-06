import { ProjectService } from "@/lib/services/project";
import { useQuery } from "@tanstack/react-query";

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
}

export const useDescribeProjectEntry = ({ projectId, entryId }: UseDescribeProjectEntryProps) => {
  return useQuery({
    queryKey: [USE_DESCRIBE_PROJECT_ENTRY_QUERY_KEY, projectId, entryId],
    queryFn: () => describeProjectEntryFn({ projectId, entryId }),
  });
};
