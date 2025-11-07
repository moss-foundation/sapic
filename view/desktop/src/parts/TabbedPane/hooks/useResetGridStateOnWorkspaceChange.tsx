import { useEffect } from "react";

import { useActiveWorkspace } from "@/hooks";
import { useGetTabbedPane } from "@/hooks/sharedStorage/layout/tabbedPane/useGetTabbedPane";
import { useTabbedPaneStore } from "@/store/tabbedPane";

export const useResetGridStateOnWorkspaceChange = () => {
  const { activeWorkspaceId } = useActiveWorkspace();
  const { api, addOrFocusPanel } = useTabbedPaneStore();
  const { data: tabbedPane } = useGetTabbedPane();

  useEffect(() => {
    if (!api || !tabbedPane?.gridState) return;

    try {
      // api.clear();
      api.fromJSON(tabbedPane.gridState);

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
  }, [activeWorkspaceId, addOrFocusPanel, api, tabbedPane]);
};
