import { useEffect, useRef } from "react";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { useDescribeWorkspaceState } from "./useDescribeWorkspaceState";
import { useUpdateSidebarPartState } from "@/hooks/appState/useUpdateSidebarPartState";
import { useActiveWorkspace } from "./useActiveWorkspace";

export const useWorkspaceSidebarState = () => {
  const workspace = useActiveWorkspace();
  const workspaceId = workspace?.id || null;

  const { sideBar, sideBarPosition, initialize } = useAppResizableLayoutStore();
  const { mutate: updateSidebarPartState } = useUpdateSidebarPartState();

  const hasInitializedSidebarState = useRef<string | null>(null);
  const canUpdatePartState = useRef(false);

  // Fetch workspace state only when we have an active workspace
  const { data: workspaceState, isFetched } = useDescribeWorkspaceState({
    enabled: !!workspaceId,
  });

  // Initialize sidebar state from workspace data
  useEffect(() => {
    if (!workspaceId || !isFetched) return;

    if (hasInitializedSidebarState.current === workspaceId) return;

    if (workspaceState?.sidebar) {
      initialize({
        sideBar: {
          width: workspaceState.sidebar.size,
          visible: workspaceState.sidebar.visible,
        },
        // Don't modify bottomPane here, it should be handled separately
        bottomPane: {
          // Keep existing values
        },
      });

      useAppResizableLayoutStore.getState().setSideBarPosition(workspaceState.sidebar.position);
    }

    hasInitializedSidebarState.current = workspaceId;

    setTimeout(() => {
      canUpdatePartState.current = true;
    }, 100);
  }, [workspaceId, workspaceState?.sidebar, isFetched, initialize]);

  // Reset initialization when workspace changes
  useEffect(() => {
    if (hasInitializedSidebarState.current !== workspaceId) {
      hasInitializedSidebarState.current = null;
      canUpdatePartState.current = false;
    }
  }, [workspaceId]);

  // Save sidebar state changes to backend (only when workspace is active)
  useEffect(() => {
    if (!workspaceId || !canUpdatePartState.current) return;

    updateSidebarPartState({
      size: sideBar.width,
      visible: sideBar.visible,
      position: sideBarPosition,
    });
  }, [workspaceId, sideBar.width, sideBar.visible, sideBarPosition, updateSidebarPartState]);

  return {
    hasWorkspace: !!workspaceId,
    isInitialized: hasInitializedSidebarState.current === workspaceId,
  };
};
