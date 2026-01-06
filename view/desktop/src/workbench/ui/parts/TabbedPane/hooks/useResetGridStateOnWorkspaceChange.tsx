import { useEffect, useEffectEvent } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { useGetLayout } from "@/workbench/adapters";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

export const useResetGridStateOnWorkspaceChange = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { api, addOrFocusPanel } = useTabbedPaneStore();
  const { data: layout, isFetching: isFetchingLayout } = useGetLayout();

  const updateGridState = useEffectEvent(() => {
    if (!api || !layout?.tabbedPaneState.gridState) return;

    try {
      api.clear();
      if (!currentWorkspaceId) {
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
    updateGridState();

    //we only want to run this effect when the workspace changes
    //but there is a race condition where the workspace changes before the layout is fetched
    //so we need to check if the layout is fetching
  }, [currentWorkspaceId, isFetchingLayout]);
};
