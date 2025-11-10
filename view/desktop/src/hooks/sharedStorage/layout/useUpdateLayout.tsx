import { toMerged } from "es-toolkit";
import { SerializedDockview } from "moss-tabs";

import { defaultLayoutState } from "@/constants/layoutPositions";
import { sharedStorageService } from "@/lib/services/sharedStorage";
import { JsonValue } from "@repo/moss-bindingutils";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { LayoutStateInput, LayoutStateOutput } from "./types";
import { USE_GET_LAYOUT_QUERY_KEY, useGetLayout } from "./useGetLayout";

export const USE_UPDATE_LAYOUT_MUTATION_KEY = "updateLayout";

interface UseUpdateLayoutProps {
  layout: LayoutStateInput;
  workspaceId?: string;
}

export const useUpdateLayout = () => {
  const queryClient = useQueryClient();

  const { data: currentLayout } = useGetLayout();

  return useMutation({
    mutationKey: [USE_UPDATE_LAYOUT_MUTATION_KEY],
    mutationFn: async ({ layout: newLayout, workspaceId }: UseUpdateLayoutProps): Promise<void> => {
      if (!newLayout) return;

      console.log("newLayout", newLayout);
      const updatedLayout = toMerged(currentLayout ?? defaultLayoutState, newLayout);

      if (newLayout.tabbedPaneState?.gridState) {
        updatedLayout.tabbedPaneState.gridState = newLayout.tabbedPaneState.gridState as unknown as SerializedDockview;
      }

      console.log("updatedLayout 2", updatedLayout);
      return await sharedStorageService.putItem("layout", updatedLayout as unknown as JsonValue, workspaceId);
    },
    onSuccess: (_, { layout: newLayout, workspaceId }) => {
      queryClient.setQueryData([USE_GET_LAYOUT_QUERY_KEY, workspaceId], (old: LayoutStateOutput | undefined) => {
        if (!old) return defaultLayoutState;

        const updatedLayout = toMerged(old ?? defaultLayoutState, newLayout);

        if (newLayout.tabbedPaneState?.gridState) {
          updatedLayout.tabbedPaneState.gridState = newLayout.tabbedPaneState
            .gridState as unknown as SerializedDockview;
        }

        return updatedLayout;
      });
    },
  });
};
