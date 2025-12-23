import { useActiveWorkspace } from "@/hooks";
import { defaultLayoutState } from "@/workbench/domains/layout/defaults";
import { layoutService } from "@/workbench/domains/layout/service";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_LAYOUT_QUERY_KEY = "getLayout";

export const useGetLayout = (workspaceId?: string) => {
  const { activeWorkspaceId } = useActiveWorkspace();
  const workspaceIdToUse = workspaceId || activeWorkspaceId;

  return useQuery({
    queryKey: [USE_GET_LAYOUT_QUERY_KEY, workspaceIdToUse],
    queryFn: async () => {
      return await layoutService.getLayout(workspaceIdToUse!);
    },
    enabled: !!workspaceIdToUse,
    placeholderData: defaultLayoutState,
  });
};
