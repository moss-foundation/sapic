import { useEffect, useState } from "react";

import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { useWorkspaceState } from "@/hooks/appState/useWorkspaceState";
import { useDescribeWorkspaceState } from "@/hooks/workspaces/useDescribeWorkspaceState";

export interface WindowPreparationState {
  isPreparing: boolean;
  isInWorkspace: boolean;
}

export const usePrepareWindow = (): WindowPreparationState => {
  const [isPreparing, setIsPreparing] = useState(true);
  const { initialize } = useAppResizableLayoutStore();

  // Determine if we're in a workspace using app state
  const { state: workspaceState, isLoading: isWorkspaceStateLoading } = useWorkspaceState();
  const isInWorkspace = workspaceState === "inWorkspace";

  // Only call useDescribeWorkspaceState if we're in a workspace
  const { isFetched: isWorkspaceLayoutFetched, data: layout } = useDescribeWorkspaceState({
    enabled: isInWorkspace,
  });

  useEffect(() => {
    // If workspace state loaded and we're not in a workspace, we're ready
    if (!isWorkspaceStateLoading && !isInWorkspace) {
      setIsPreparing(false);
      return;
    }

    // If we're in a workspace, wait for workspace layout to be loaded
    if (isInWorkspace && isWorkspaceLayoutFetched) {
      setIsPreparing(false);

      if (layout) {
        initialize({
          sideBar: {
            width: layout?.sidebar?.preferredSize,
            visible: layout?.sidebar?.isVisible,
          },
          bottomPane: {
            height: layout?.panel?.preferredSize,
            visible: layout?.panel?.isVisible,
          },
        });
      }
    }
  }, [initialize, isWorkspaceStateLoading, isInWorkspace, isWorkspaceLayoutFetched, layout]);

  return { isPreparing, isInWorkspace };
};
