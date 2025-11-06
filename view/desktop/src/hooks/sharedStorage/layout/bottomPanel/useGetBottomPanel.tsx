import { defaultBottomPanePanel } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks/workspace";
import { sharedStorageService } from "@/lib/services/sharedStorageService";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_BOTTOM_PANEL_QUERY_KEY = "getBottomPanel";

export interface BottomPanel {
  height: number;
  visible: boolean;
  minHeight: number;
  maxHeight: number;
}

const queryFn = async (activeWorkspaceId?: string): Promise<BottomPanel> => {
  if (!activeWorkspaceId) {
    return {
      height: defaultBottomPanePanel.height,
      visible: defaultBottomPanePanel.visible,
      minHeight: defaultBottomPanePanel.minHeight,
      maxHeight: defaultBottomPanePanel.maxHeight,
    };
  }

  const bottomPaneHeight = (await sharedStorageService.getItem("bottomPaneHeight", activeWorkspaceId))?.value as
    | number
    | undefined;
  const bottomPaneVisible = (await sharedStorageService.getItem("bottomPaneVisible", activeWorkspaceId))?.value as
    | boolean
    | undefined;

  return {
    height: bottomPaneHeight ?? defaultBottomPanePanel.height,
    visible: bottomPaneVisible ?? defaultBottomPanePanel.visible,
    minHeight: defaultBottomPanePanel.minHeight,
    maxHeight: defaultBottomPanePanel.maxHeight,
  };
};

export const useGetBottomPanel = () => {
  const { activeWorkspaceId } = useActiveWorkspace();

  return useQuery<BottomPanel, Error>({
    queryKey: [USE_GET_BOTTOM_PANEL_QUERY_KEY, activeWorkspaceId],
    queryFn: () => queryFn(activeWorkspaceId),
    enabled: !!activeWorkspaceId,
  });
};
