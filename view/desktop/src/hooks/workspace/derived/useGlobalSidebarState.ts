import { useEffect, useRef } from "react";

import { SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { SidebarPosition } from "@repo/moss-workspace";

import { useActiveWorkspace } from "./useActiveWorkspace";

const GLOBAL_SIDEBAR_DEFAULTS = {
  width: 255,
  visible: true,
  position: SIDEBAR_POSITION.LEFT as SidebarPosition,
};

export const useGlobalSidebarState = () => {
  const { hasActiveWorkspace } = useActiveWorkspace();

  const { initialize, setSideBarPosition } = useAppResizableLayoutStore();
  const hasInitializedGlobalState = useRef(false);

  // Initialize global sidebar state when no workspace is active
  useEffect(() => {
    if (hasActiveWorkspace) {
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
  }, [hasActiveWorkspace, initialize, setSideBarPosition]);

  return {
    hasActiveWorkspace,
    isGlobalState: !hasActiveWorkspace && hasInitializedGlobalState.current,
  };
};
