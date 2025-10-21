import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar } from "@/components";
import DesignModeToggle from "@/components/DesignModeToggle";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import { SidebarWorkspaceContent } from "@/parts/SideBar/SidebarWorkspaceContent";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";

export interface BaseSidebarProps {
  className?: string;
  children?: ReactNode;
}

export const BaseSidebar = ({ className, children }: BaseSidebarProps) => {
  const sideBarPosition = useAppResizableLayoutStore((state) => state.sideBarPosition);

  return (
    <div
      className={cn(
        "background-(--moss-secondary-background) flex h-full grow flex-col",
        {
          "border-l border-(--moss-border)": sideBarPosition === SIDEBAR_POSITION.LEFT,
        },
        className
      )}
    >
      {children}
    </div>
  );
};

export const Sidebar = () => {
  const { data: projectSessionState } = useGetProjectSessionState();
  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();

  const { data: workspaceState, isFetched, isSuccess } = useDescribeWorkspaceState();
  const { updateFromWorkspaceState, resetToDefaults } = useActivityBarStore();
  const lastRestoredWorkspaceId = useRef<string | null>(null);

  const lastActiveGroupRef = useRef<string | null>(null);

  useEffect(() => {
    if (projectSessionState?.lastActiveGroup) {
      lastActiveGroupRef.current = projectSessionState.lastActiveGroup;
    }
  }, [projectSessionState?.lastActiveGroup]);

  // Reset activity bar state when workspace changes (before restoration)
  useEffect(() => {
    if (lastRestoredWorkspaceId.current !== activeWorkspaceId) {
      // Reset to default state before loading new workspace state
      if (activeWorkspaceId) {
        // Will be restored from workspace state
        resetToDefaults();
        lastRestoredWorkspaceId.current = null;
      } else {
        // Reset to default when no workspace
        resetToDefaults();
        lastRestoredWorkspaceId.current = activeWorkspaceId;
      }
    }
  }, [activeWorkspaceId, resetToDefaults]);

  // Restore activity bar state from workspace state
  useEffect(() => {
    if (!activeWorkspaceId || !isFetched || !isSuccess || !workspaceState?.layouts.activitybar) return;

    if (lastRestoredWorkspaceId.current === activeWorkspaceId) return;

    // Only restore if we have fresh workspace state for this workspace
    // Add a small delay to ensure workspace switching is complete
    const timeoutId = setTimeout(() => {
      updateFromWorkspaceState(workspaceState.layouts.activitybar!);
      lastRestoredWorkspaceId.current = activeWorkspaceId;
    }, 100);

    return () => clearTimeout(timeoutId);
  }, [activeWorkspaceId, workspaceState?.layouts.activitybar, isFetched, isSuccess, updateFromWorkspaceState]);

  const { position } = useActivityBarStore();

  const activeItem = useActivityBarStore((state) => state.getActiveItem());

  const activeGroupId = activeItem?.id || "default";

  const sidebarContent = hasActiveWorkspace ? (
    <SidebarWorkspaceContent groupId={activeGroupId} />
  ) : (
    <EmptyWorkspace inSidebar={true} />
  );

  if (position === ACTIVITYBAR_POSITION.TOP) {
    return (
      <BaseSidebar>
        <ActivityBar />
        <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
        {hasActiveWorkspace && (
          <div className="z-20 flex w-full justify-end border-t border-t-(--moss-border) px-1 py-2">
            <DesignModeToggle />
          </div>
        )}
      </BaseSidebar>
    );
  }

  if (position === ACTIVITYBAR_POSITION.BOTTOM) {
    return (
      <BaseSidebar>
        <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
        {hasActiveWorkspace && (
          <div className="z-20 flex w-full justify-end border-t border-t-(--moss-border) px-1 py-2">
            <DesignModeToggle />
          </div>
        )}
        <ActivityBar />
      </BaseSidebar>
    );
  }

  return (
    <BaseSidebar>
      <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
      {hasActiveWorkspace && (
        <div className="z-20 flex w-full justify-end border-t border-t-(--moss-border) px-1 py-2">
          <DesignModeToggle />
        </div>
      )}
    </BaseSidebar>
  );
};

export default Sidebar;
