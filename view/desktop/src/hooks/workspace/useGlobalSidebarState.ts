import { useEffect, useRef } from "react";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { useActiveWorkspace } from "./useActiveWorkspace";
import { SidebarPosition } from "@repo/moss-workspace";
import { SIDEBAR_POSITION } from "@/constants/layoutPositions";

const GLOBAL_SIDEBAR_DEFAULTS = {
  width: 255,
  visible: true,
  position: SIDEBAR_POSITION.LEFT as SidebarPosition,
};

export const useGlobalSidebarState = () => {
  const workspace = useActiveWorkspace();
  const hasWorkspace = !!workspace;

  const { initialize, setSideBarPosition } = useAppResizableLayoutStore();
  const hasInitializedGlobalState = useRef(false);

  // Initialize global sidebar state when no workspace is active
  useEffect(() => {
    if (hasWorkspace) {
      // Reset flag when workspace becomes active
      hasInitializedGlobalState.current = false;
      return;
    }

    if (hasInitializedGlobalState.current) return;

    initialize({
      sideBar: {
        width: GLOBAL_SIDEBAR_DEFAULTS.width,
        visible: GLOBAL_SIDEBAR_DEFAULTS.visible,
      },
      bottomPane: {},
    });

    setSideBarPosition(GLOBAL_SIDEBAR_DEFAULTS.position);
    hasInitializedGlobalState.current = true;
  }, [hasWorkspace, initialize, setSideBarPosition]);

  return {
    hasWorkspace,
    isGlobalState: !hasWorkspace && hasInitializedGlobalState.current,
  };
};
