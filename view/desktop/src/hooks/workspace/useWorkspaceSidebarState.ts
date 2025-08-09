import { useEffect, useRef } from "react";

import { useUpdateSidebarPartState } from "@/hooks/appState/useUpdateSidebarPartState";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import { useActiveWorkspace } from "./useActiveWorkspace";
import { useDescribeWorkspaceState } from "./useDescribeWorkspaceState";

export const useWorkspaceSidebarState = () => {
  const workspace = useActiveWorkspace();
  const workspaceId = workspace?.id || null;

  const { sideBar, sideBarPosition, initialize, setSideBarPosition } = useAppResizableLayoutStore();
  const { mutate: updateSidebarPartState } = useUpdateSidebarPartState();

  const lastInitializedWorkspaceId = useRef<string | null>(null);
  const canUpdatePartState = useRef(false);
  const isTransitioning = useRef(false);

  // Fetch workspace state only when we have an active workspace
  const { data: workspaceState, isFetched, isSuccess } = useDescribeWorkspaceState();

  // Reset state tracking when workspace changes
  useEffect(() => {
    if (lastInitializedWorkspaceId.current !== workspaceId) {
      lastInitializedWorkspaceId.current = null;
      canUpdatePartState.current = false;
      isTransitioning.current = true;
    }
  }, [workspaceId]);

  // Initialize sidebar state from workspace data
  useEffect(() => {
    if (!workspaceId || !isFetched || !isSuccess) {
      canUpdatePartState.current = false;
      isTransitioning.current = true;
      return;
    }

    if (lastInitializedWorkspaceId.current === workspaceId) return;

    if (workspaceState?.sidebar) {
      initialize({
        sideBar: {
          width: workspaceState.sidebar.size,
          visible: workspaceState.sidebar.visible,
        },
        // Don't modify bottomPane here, it should be handled separately
        bottomPane: {},
      });

      setSideBarPosition(workspaceState.sidebar.position);
    }

    lastInitializedWorkspaceId.current = workspaceId;

    setTimeout(() => {
      canUpdatePartState.current = true;
      isTransitioning.current = false;
    }, 150);
  }, [workspaceId, workspaceState?.sidebar, isFetched, isSuccess, initialize, setSideBarPosition]);

  // Save sidebar state changes to backend (only when workspace is active and initialization is complete)
  useEffect(() => {
    if (!workspaceId || !canUpdatePartState.current || isTransitioning.current) return;

    updateSidebarPartState({
      size: sideBar.width,
      visible: sideBar.visible,
      position: sideBarPosition,
    });
  }, [workspaceId, sideBar.width, sideBar.visible, sideBarPosition, updateSidebarPartState]);

  return {
    hasWorkspace: !!workspaceId,
    isInitialized: lastInitializedWorkspaceId.current === workspaceId && !isTransitioning.current,
  };
};
