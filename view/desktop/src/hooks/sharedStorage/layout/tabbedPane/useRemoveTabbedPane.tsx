import { sharedStorageService } from "@/lib/services/sharedStorage";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_TABBED_PANE_QUERY_KEY } from "./useGetTabbedPane";

export const USE_REMOVE_TABBED_PANE_MUTATION_KEY = "removeTabbedPane";

const mutationFn = async (workspaceId: string) => {
  await sharedStorageService.removeItem("gridState", workspaceId);
};

export const useRemoveTabbedPane = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, string>({
    mutationKey: [USE_REMOVE_TABBED_PANE_MUTATION_KEY],
    mutationFn,
    onSuccess: (_, workspaceId) => {
      queryClient.removeQueries({ queryKey: [USE_GET_TABBED_PANE_QUERY_KEY, workspaceId] });
    },
  });
};
