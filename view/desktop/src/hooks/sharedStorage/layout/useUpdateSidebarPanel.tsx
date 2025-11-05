import { defaultSidebarPanel } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks/workspace";
import { sharedStorageService } from "@/lib/services/sharedStorageService";
import { SidebarPosition } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { SidebarPanel, USE_GET_SIDEBAR_PANEL_QUERY_KEY } from "./useGetSidebarPanel";

export const USE_UPDATE_SIDEBAR_PANEL_MUTATION_KEY = "updateSidebarPanel";

interface UpdateSidebarPanelParams {
  position?: SidebarPosition;
  size?: number;
  visible?: boolean;
}

export const useUpdateSidebarPanel = () => {
  const queryClient = useQueryClient();
  const { activeWorkspaceId } = useActiveWorkspace();

  return useMutation<void, Error, UpdateSidebarPanelParams>({
    mutationKey: [USE_UPDATE_SIDEBAR_PANEL_MUTATION_KEY],

    mutationFn: async ({ position, size, visible }): Promise<void> => {
      if (!activeWorkspaceId) return;

      if (position) {
        await sharedStorageService.putItem("sidebarPosition", position, activeWorkspaceId);
      }
      if (size) {
        await sharedStorageService.putItem("sidebarSize", size, activeWorkspaceId);
      }
      if (visible) {
        await sharedStorageService.putItem("sidebarVisible", visible, activeWorkspaceId);
      }
    },
    onSuccess: async (_, variables) => {
      queryClient.setQueryData<SidebarPanel>([USE_GET_SIDEBAR_PANEL_QUERY_KEY, activeWorkspaceId], (old) => {
        console.log({
          position: variables.position ?? old?.position ?? defaultSidebarPanel.position,
          size: variables.size ?? old?.size ?? defaultSidebarPanel.size,
          visible: variables.visible ?? old?.visible ?? defaultSidebarPanel.visible,
          minWidth: old?.minWidth ?? defaultSidebarPanel.minWidth,
          maxWidth: old?.maxWidth ?? defaultSidebarPanel.maxWidth,
        });

        return {
          ...old,
          position: variables.position ?? old?.position ?? defaultSidebarPanel.position,
          size: variables.size ?? old?.size ?? defaultSidebarPanel.size,
          visible: variables.visible ?? old?.visible ?? defaultSidebarPanel.visible,
          minWidth: old?.minWidth ?? defaultSidebarPanel.minWidth,
          maxWidth: old?.maxWidth ?? defaultSidebarPanel.maxWidth,
        };
      });
    },
  });
};
