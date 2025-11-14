import { defaultLayoutState } from "@/defaults/layout";
import { useActiveWorkspace } from "@/hooks/workspace/derived/useActiveWorkspace";
import { layoutService, LayoutStateOutput } from "@/workbench/domains/layout/service";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_LAYOUT_QUERY_KEY = "getLayout";

const queryFn = async (activeWorkspaceId?: string): Promise<LayoutStateOutput> => {
  if (!activeWorkspaceId) {
    return defaultLayoutState;
  }

  const layout = (await layoutService.getLayout(activeWorkspaceId))?.value as unknown as LayoutStateOutput;

  return layout;
};

export const useGetLayout = () => {
  const { activeWorkspaceId } = useActiveWorkspace();

  return useQuery({
    queryKey: [USE_GET_LAYOUT_QUERY_KEY, activeWorkspaceId],
    queryFn: () => queryFn(activeWorkspaceId),
    placeholderData: defaultLayoutState,
  });
};
