import { useEffect } from "react";

import { useActiveWorkspace } from "@/hooks";
import { useGetTabbedPane } from "@/hooks/sharedStorage/layout/tabbedPane/useGetTabbedPane";
import { useTabbedPaneStore } from "@/store/tabbedPane";

export const useResetGridStateOnWorkspaceChange = () => {
  const { activeWorkspaceId } = useActiveWorkspace();
  const { api } = useTabbedPaneStore();
  const { data: tabbedPane } = useGetTabbedPane();

  useEffect(() => {
    if (!activeWorkspaceId || !api || !tabbedPane) return;
    api.fromJSON(tabbedPane.gridState);
  }, [activeWorkspaceId, api, tabbedPane]);
};
