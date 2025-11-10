import { defaultBottomPanePanelState } from "@/constants/layoutStates";
import { sharedStorageService } from "@/lib/services/sharedStorage";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { BottomPanel, USE_GET_BOTTOM_PANEL_QUERY_KEY } from "./useGetBottomPanel";

export const USE_UPDATE_BOTTOM_PANEL_MUTATION_KEY = "updateBottomPanel";

interface UpdateBottomPanelParams {
  height?: number;
  visible?: boolean;
  workspaceId?: string;
}

const mutationFn = async ({ height, visible, workspaceId }: UpdateBottomPanelParams): Promise<void> => {
  if ((!height && !visible) || (!height && !visible && !workspaceId)) return;

  if (height) await sharedStorageService.putItem("bottomPaneHeight", height, workspaceId);
  if (visible) await sharedStorageService.putItem("bottomPaneVisible", visible, workspaceId);
};

export const useUpdateBottomPanel = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, UpdateBottomPanelParams>({
    mutationKey: [USE_UPDATE_BOTTOM_PANEL_MUTATION_KEY],
    mutationFn,
    onSuccess: async (_, variables) => {
      queryClient.setQueryData<BottomPanel>([USE_GET_BOTTOM_PANEL_QUERY_KEY, variables.workspaceId], (old) => {
        return {
          ...old,
          height: variables.height ?? old?.height ?? defaultBottomPanePanelState.height,
          visible: variables.visible ?? old?.visible ?? defaultBottomPanePanelState.visible,
          minHeight: old?.minHeight ?? defaultBottomPanePanelState.minHeight,
          maxHeight: old?.maxHeight ?? defaultBottomPanePanelState.maxHeight,
        };
      });
    },
  });
};
