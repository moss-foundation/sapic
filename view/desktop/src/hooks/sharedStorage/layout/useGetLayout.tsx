import { defaultLayout } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks/workspace/derived/useActiveWorkspace";
import { sharedStorageService } from "@/lib/services/sharedStorage";
import { useQuery } from "@tanstack/react-query";

import { LayoutOutput } from "./types";

export const USE_GET_LAYOUT_QUERY_KEY = "getLayout";

const queryFn = async (activeWorkspaceId?: string): Promise<LayoutOutput> => {
  if (!activeWorkspaceId) {
    return defaultLayout;
  }

  const layout = (await sharedStorageService.getItem("layout", activeWorkspaceId))?.value as unknown as LayoutOutput;
  return layout;
};

export const useGetLayout = () => {
  const { activeWorkspaceId } = useActiveWorkspace();

  return useQuery({
    queryKey: [USE_GET_LAYOUT_QUERY_KEY, activeWorkspaceId],
    queryFn: () => queryFn(activeWorkspaceId),
    placeholderData: defaultLayout,
  });
};
