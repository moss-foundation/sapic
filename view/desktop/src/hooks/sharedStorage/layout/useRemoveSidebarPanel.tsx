import { sharedStorageService } from "@/lib/services";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_SIDEBAR_PANEL_QUERY_KEY } from "./useGetSidebarPanel";

export const USE_REMOVE_SIDEBAR_PANEL_MUTATION_KEY = "removeSidebarPanel";

const mutationFn = async (workspaceId: string) => {
  await sharedStorageService.removeItem("sidebarPosition", workspaceId);

  await sharedStorageService.removeItem("sidebarSize", workspaceId);
  await sharedStorageService.removeItem("sidebarVisible", workspaceId);
  await sharedStorageService.removeItem("sidebarMinWidth", workspaceId);
  await sharedStorageService.removeItem("sidebarMaxWidth", workspaceId);

  await sharedStorageService.removeItem("bottomPaneHeight", workspaceId);
  await sharedStorageService.removeItem("bottomPaneMinHeight", workspaceId);
  await sharedStorageService.removeItem("bottomPaneMaxHeight", workspaceId);
  await sharedStorageService.removeItem("bottomPaneVisible", workspaceId);

  await sharedStorageService.removeItem("gridState", workspaceId);
};

export const useRemoveSidebarPanel = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, string>({
    mutationKey: [USE_REMOVE_SIDEBAR_PANEL_MUTATION_KEY],
    mutationFn: mutationFn,
    onSuccess: (_, workspaceId) => {
      queryClient.removeQueries({ queryKey: [USE_GET_SIDEBAR_PANEL_QUERY_KEY, workspaceId] });
      //TODO: Remove bottom pane queries
      //TODO: Remove grid state queries
    },
  });
};
