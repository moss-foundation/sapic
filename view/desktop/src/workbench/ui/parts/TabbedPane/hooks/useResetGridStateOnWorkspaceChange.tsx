import { useEffect, useEffectEvent } from "react";

import { useActiveWorkspace } from "@/hooks";
import { useGetLayout } from "@/hooks/workbench/layout/useGetLayout";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

export const useResetGridStateOnWorkspaceChange = () => {
  const { activeWorkspaceId } = useActiveWorkspace();
  const { api, addOrFocusPanel } = useTabbedPaneStore();
  const { data: layout, isFetching: isFetchingLayout } = useGetLayout();

  const updateGridState = useEffectEvent((activeWorkspaceId) => {
    if (!api || !layout?.tabbedPaneState.gridState) return;

    try {
      api.clear();
      if (!activeWorkspaceId) {
        addOrFocusPanel({
          id: "Welcome",
          title: "Welcome",
          component: "WelcomeView",
        });
      } else {
        api.fromJSON(layout?.tabbedPaneState.gridState);
      }
    } catch (error) {
      console.error("Error resetting grid state on workspace change:", error);
    }
  });

  useEffect(() => {
    if (isFetchingLayout) return;
    updateGridState(activeWorkspaceId);

    //we only want to run this effect when the workspace changes
    //but there is a race condition where the workspace changes before the layout is fetched
    //so we need to check if the layout is fetching
  }, [activeWorkspaceId, isFetchingLayout]);
};
