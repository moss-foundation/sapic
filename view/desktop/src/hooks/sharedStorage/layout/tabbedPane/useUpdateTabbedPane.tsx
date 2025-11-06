import { SerializedDockview } from "moss-tabs";

import { sharedStorageService } from "@/lib/services/sharedStorageService";
import { JsonValue } from "@repo/moss-bindingutils";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_TABBED_PANE_QUERY_KEY } from "./useGetTabbedPane";

export const USE_UPDATE_TABBED_PANE_MUTATION_KEY = "updateTabbedPane";

interface UseUpdateTabbedPaneProps {
  gridState: SerializedDockview;
  workspaceId?: string;
}

const mutationFn = async ({ gridState, workspaceId }: UseUpdateTabbedPaneProps) => {
  if (!gridState) return;

  return await sharedStorageService.putItem("gridState", gridState as unknown as JsonValue, workspaceId);
};

export const useUpdateTabbedPane = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { gridState: SerializedDockview; workspaceId?: string }>({
    mutationKey: [USE_UPDATE_TABBED_PANE_MUTATION_KEY],
    mutationFn,
    onSuccess: async (_, { gridState, workspaceId }) => {
      queryClient.setQueryData<UseUpdateTabbedPaneProps>([USE_GET_TABBED_PANE_QUERY_KEY, workspaceId], (old) => {
        return {
          ...old,
          gridState: gridState,
        };
      });
    },
  });
};
