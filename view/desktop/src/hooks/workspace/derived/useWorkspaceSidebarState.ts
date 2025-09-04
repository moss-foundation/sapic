import { useEffect, useRef } from "react";

import { useUpdateSidebarPartState } from "@/hooks/app/useUpdateSidebarPartState";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import { useDescribeWorkspaceState } from "../useDescribeWorkspaceState";
import { useActiveWorkspace } from "./useActiveWorkspace";

export const useWorkspaceSidebarState = () => {
  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();

  const { sideBar, sideBarPosition, initialize, setSideBarPosition } = useAppResizableLayoutStore();
  const { mutate: updateSidebarPartState } = useUpdateSidebarPartState();

  const lastInitializedWorkspaceId = useRef<string | null>(null);
  const canUpdatePartState = useRef(false);
  const isTransitioning = useRef(false);

  // Fetch workspace state only when we have an active workspace
  const { data: workspaceState, isFetched, isSuccess } = useDescribeWorkspaceState();

  // Reset state tracking when workspace changes
  useEffect(() => {
    if (lastInitializedWorkspaceId.current !== activeWorkspaceId) {
      lastInitializedWorkspaceId.current = null;
      canUpdatePartState.current = false;
      isTransitioning.current = true;
    }
  }, [activeWorkspaceId]);

  // Initialize sidebar state from workspace data
  useEffect(() => {
    if (!activeWorkspaceId || !isFetched || !isSuccess) {
      canUpdatePartState.current = false;
      isTransitioning.current = true;
      return;
    }

    if (lastInitializedWorkspaceId.current === activeWorkspaceId) return;

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

    lastInitializedWorkspaceId.current = activeWorkspaceId;

    setTimeout(() => {
      canUpdatePartState.current = true;
      isTransitioning.current = false;
    }, 150);
  }, [activeWorkspaceId, workspaceState?.sidebar, isFetched, isSuccess, initialize, setSideBarPosition]);

  // Save sidebar state changes to backend (only when workspace is active and initialization is complete)
  useEffect(() => {
    if (!activeWorkspaceId || !canUpdatePartState.current || isTransitioning.current) return;

    updateSidebarPartState({
      size: sideBar.width,
      visible: sideBar.visible,
      position: sideBarPosition,
    });
  }, [activeWorkspaceId, sideBar.width, sideBar.visible, sideBarPosition, updateSidebarPartState]);

  return {
    hasActiveWorkspace,
    isInitialized: lastInitializedWorkspaceId.current === activeWorkspaceId && !isTransitioning.current,
  };
};
