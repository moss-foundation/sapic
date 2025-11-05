import { sharedStorageService } from "@/lib/services";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_BOTTOM_PANEL_QUERY_KEY } from "./useGetBottomPanel";

export const USE_REMOVE_BOTTOM_PANEL_MUTATION_KEY = "removeBottomPanel";

const mutationFn = async (workspaceId: string) => {
  await sharedStorageService.removeItem("bottomPaneHeight", workspaceId);
  await sharedStorageService.removeItem("bottomPaneMinHeight", workspaceId);
  await sharedStorageService.removeItem("bottomPaneMaxHeight", workspaceId);
  await sharedStorageService.removeItem("bottomPaneVisible", workspaceId);
};

export const useRemoveBottomPanel = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, string>({
    mutationKey: [USE_REMOVE_BOTTOM_PANEL_MUTATION_KEY],
    mutationFn: mutationFn,
    onSuccess: (_, workspaceId) => {
      queryClient.removeQueries({ queryKey: [USE_GET_BOTTOM_PANEL_QUERY_KEY, workspaceId] });
    },
  });
};
