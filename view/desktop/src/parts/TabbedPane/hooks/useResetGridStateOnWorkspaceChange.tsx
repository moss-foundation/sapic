import { useEffect, useEffectEvent } from "react";

import { useActiveWorkspace } from "@/hooks";
import { useGetLayout } from "@/hooks/sharedStorage/layout/useGetLayout";
import { useTabbedPaneStore } from "@/store/tabbedPane";

export const useResetGridStateOnWorkspaceChange = () => {
  const { activeWorkspaceId } = useActiveWorkspace();
  const { api, addOrFocusPanel } = useTabbedPaneStore();
  const { data: layout } = useGetLayout();

  const updateGridState = useEffectEvent((activeWorkspaceId) => {
    if (!api || !layout?.tabbedPaneState.gridState) return;

    try {
      api.clear();
      api.fromJSON(layout?.tabbedPaneState.gridState);

      if (!activeWorkspaceId) {
        addOrFocusPanel({
          id: "Welcome",
          component: "Welcome",
          title: "Welcome",
        });
      }
    } catch (error) {
      console.error("Error resetting grid state on workspace change:", error);
    }
  });

  useEffect(() => {
    updateGridState(activeWorkspaceId);
  }, [activeWorkspaceId]);
};
