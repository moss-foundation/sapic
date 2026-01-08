import { useEffect } from "react";

import { useCurrentWorkspace } from "@/hooks/workspace";
import { USE_GET_LAYOUT_QUERY_KEY } from "@/workbench/adapters";
import { layoutService } from "@/workbench/domains/layout/service";
import { useQueryClient } from "@tanstack/react-query";

export const usePrefetchWorkspaceLayout = () => {
  const queryClient = useQueryClient();
  const { currentWorkspaceId } = useCurrentWorkspace();

  useEffect(() => {
    if (currentWorkspaceId) {
      queryClient.prefetchQuery({
        queryKey: [USE_GET_LAYOUT_QUERY_KEY, currentWorkspaceId],
        queryFn: () => layoutService.getLayout(currentWorkspaceId),
      });
    }
  }, [currentWorkspaceId, queryClient]);
};
