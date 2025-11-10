import { defaultSidebarPanelState } from "@/constants/layoutPositions";
import { sharedStorageService } from "@/lib/services/sharedStorage";
import { SidebarPosition } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { SidebarPanel, USE_GET_SIDEBAR_PANEL_QUERY_KEY } from "./useGetSidebarPanel";

export const USE_UPDATE_SIDEBAR_PANEL_MUTATION_KEY = "updateSidebarPanel";

interface UpdateSidebarPanelParams {
  position?: SidebarPosition;
  size?: number;
  visible?: boolean;
  workspaceId?: string;
}

const mutationFn = async ({ position, size, visible, workspaceId }: UpdateSidebarPanelParams): Promise<void> => {
  if ((!position && !size && !visible) || (!position && !size && !visible && !workspaceId)) return;
  if (position) await sharedStorageService.putItem("sidebarPosition", position, workspaceId);
  if (size) await sharedStorageService.putItem("sidebarSize", size, workspaceId);
  if (visible) await sharedStorageService.putItem("sidebarVisible", visible, workspaceId);
};

export const useUpdateSidebarPanel = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, UpdateSidebarPanelParams>({
    mutationKey: [USE_UPDATE_SIDEBAR_PANEL_MUTATION_KEY],
    mutationFn,
    onSuccess: async (_, variables) => {
      queryClient.setQueryData<SidebarPanel>([USE_GET_SIDEBAR_PANEL_QUERY_KEY, variables.workspaceId], (old) => {
        return {
          ...old,
          position: variables.position ?? old?.position ?? defaultSidebarPanelState.position,
          size: variables.size ?? old?.size ?? defaultSidebarPanelState.size,
          visible: variables.visible ?? old?.visible ?? defaultSidebarPanelState.visible,
          minWidth: old?.minWidth ?? defaultSidebarPanelState.minWidth,
          maxWidth: old?.maxWidth ?? defaultSidebarPanelState.maxWidth,
        };
      });
    },
  });
};
