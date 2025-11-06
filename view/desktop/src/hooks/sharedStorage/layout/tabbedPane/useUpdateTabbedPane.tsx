import { SerializedDockview } from "moss-tabs";

import { useActiveWorkspace } from "@/hooks/workspace";
import { sharedStorageService } from "@/lib/services/sharedStorageService";
import { JsonValue } from "@repo/moss-bindingutils";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_TABBED_PANE_QUERY_KEY } from "./useGetTabbedPane";

export const USE_UPDATE_TABBED_PANE_MUTATION_KEY = "updateTabbedPane";

interface UseUpdateTabbedPaneProps {
  gridState: SerializedDockview;
}

const mutateFn = async (tabbedPane: SerializedDockview, activeWorkspaceId: string | null) => {
  if (!activeWorkspaceId) return;
  await sharedStorageService.putItem("gridState", tabbedPane as unknown as JsonValue, activeWorkspaceId);
};

export const useUpdateTabbedPane = () => {
  const queryClient = useQueryClient();

  const { activeWorkspaceId } = useActiveWorkspace();

  return useMutation<void, Error, SerializedDockview>({
    mutationKey: [USE_UPDATE_TABBED_PANE_MUTATION_KEY],
    mutationFn: async (tabbedPane: SerializedDockview): Promise<void> => {
      await mutateFn(tabbedPane, activeWorkspaceId);
    },
    onSuccess: async (_, tabbedPane) => {
      queryClient.setQueryData<UseUpdateTabbedPaneProps>([USE_GET_TABBED_PANE_QUERY_KEY, activeWorkspaceId], (old) => {
        return {
          ...old,
          gridState: tabbedPane,
        };
      });
    },
  });
};
