import { defaultBottomPanePanel } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks/workspace";
import { sharedStorageService } from "@/lib/services/sharedStorageService";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { BottomPanel, USE_GET_BOTTOM_PANEL_QUERY_KEY } from "./useGetBottomPanel";

export const USE_UPDATE_BOTTOM_PANEL_MUTATION_KEY = "updateBottomPanel";

interface UpdateBottomPanelParams {
  height?: number;
  visible?: boolean;
}

export const useUpdateBottomPanel = () => {
  const queryClient = useQueryClient();
  const { activeWorkspaceId } = useActiveWorkspace();

  return useMutation<void, Error, UpdateBottomPanelParams>({
    mutationKey: [USE_UPDATE_BOTTOM_PANEL_MUTATION_KEY],

    mutationFn: async ({ height, visible }): Promise<void> => {
      if (!activeWorkspaceId) return;

      if (height) {
        await sharedStorageService.putItem("bottomPaneHeight", height, activeWorkspaceId);
      }
      if (visible) {
        await sharedStorageService.putItem("bottomPaneVisible", visible, activeWorkspaceId);
      }
    },
    onSuccess: async (_, variables) => {
      queryClient.setQueryData<BottomPanel>([USE_GET_BOTTOM_PANEL_QUERY_KEY, activeWorkspaceId], (old) => {
        return {
          ...old,
          height: variables.height ?? old?.height ?? defaultBottomPanePanel.height,
          visible: variables.visible ?? old?.visible ?? defaultBottomPanePanel.visible,
          minHeight: old?.minHeight ?? defaultBottomPanePanel.minHeight,
          maxHeight: old?.maxHeight ?? defaultBottomPanePanel.maxHeight,
        };
      });
    },
  });
};
