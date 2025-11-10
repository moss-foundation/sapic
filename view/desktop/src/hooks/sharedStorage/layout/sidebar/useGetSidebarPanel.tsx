import { defaultSidebarPanelState } from "@/constants/layoutStates";
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
      position: defaultSidebarPanelState.position,
      width: defaultSidebarPanelState.width,
      visible: defaultSidebarPanelState.visible,
      minWidth: defaultSidebarPanelState.minWidth,
      maxWidth: defaultSidebarPanelState.maxWidth,
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
    position: sidebarPosition ?? defaultSidebarPanelState.position,
    width: sidebarWidth ?? defaultSidebarPanelState.width,
    visible: sidebarVisible ?? defaultSidebarPanelState.visible,
    minWidth: defaultSidebarPanelState.minWidth,
    maxWidth: defaultSidebarPanelState.maxWidth,
  };
};

export const useGetSidebarPanel = () => {
  const { activeWorkspaceId } = useActiveWorkspace();

  return useQuery<SidebarPanel, Error>({
    queryKey: [USE_GET_SIDEBAR_PANEL_QUERY_KEY, activeWorkspaceId],
    queryFn: () => queryFn(activeWorkspaceId),
  });
};
