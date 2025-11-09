import { defaultSidebarPanel } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks/workspace";
import { sharedStorageService } from "@/lib/services/sharedStorage";
import { SidebarPosition } from "@repo/moss-workspace";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_SIDEBAR_PANEL_QUERY_KEY = "getSidebarPanel";

export interface SidebarPanel {
  position: SidebarPosition;
  width: number;
  visible: boolean;
  minWidth: number;
  maxWidth: number;
}

const queryFn = async (activeWorkspaceId?: string): Promise<SidebarPanel> => {
  if (!activeWorkspaceId) {
    return {
      position: defaultSidebarPanel.position,
      width: defaultSidebarPanel.width,
      visible: defaultSidebarPanel.visible,
      minWidth: defaultSidebarPanel.minWidth,
      maxWidth: defaultSidebarPanel.maxWidth,
    };
  }

  const sidebarPosition = (await sharedStorageService.getItem("sidebarPosition", activeWorkspaceId))?.value as
    | SidebarPosition
    | undefined;
  const sidebarWidth = (await sharedStorageService.getItem("sidebarWidth", activeWorkspaceId))?.value as
    | number
    | undefined;
  const sidebarVisible = (await sharedStorageService.getItem("sidebarVisible", activeWorkspaceId))?.value as
    | boolean
    | undefined;

  return {
    position: sidebarPosition ?? defaultSidebarPanel.position,
    width: sidebarWidth ?? defaultSidebarPanel.width,
    visible: sidebarVisible ?? defaultSidebarPanel.visible,
    minWidth: defaultSidebarPanel.minWidth,
    maxWidth: defaultSidebarPanel.maxWidth,
  };
};

export const useGetSidebarPanel = () => {
  const { activeWorkspaceId } = useActiveWorkspace();

  return useQuery<SidebarPanel, Error>({
    queryKey: [USE_GET_SIDEBAR_PANEL_QUERY_KEY, activeWorkspaceId],
    queryFn: () => queryFn(activeWorkspaceId),
  });
};
