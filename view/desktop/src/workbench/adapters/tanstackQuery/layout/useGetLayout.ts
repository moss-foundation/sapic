import { layoutService } from "@/workbench/domains/layout/service";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_LAYOUT_QUERY_KEY = "getLayout";

interface UseGetLayoutProps {
  workspaceId?: string;
}

export const useGetLayout = ({ workspaceId }: UseGetLayoutProps) => {
  return useQuery({
    queryKey: [USE_GET_LAYOUT_QUERY_KEY, workspaceId],
    queryFn: async () => {
      if (!workspaceId) return;
      return await layoutService.getLayout(workspaceId);
    },
  });
};
