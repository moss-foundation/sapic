import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar } from "@/components";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import WorkspaceModeToggle from "@/components/WorkspaceModeToggle";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks";
import { useGetSidebarPanel } from "@/hooks/sharedStorage/layout/sidebar/useGetSidebarPanel";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import { SidebarWorkspaceContent } from "@/parts/SideBar/SidebarWorkspaceContent";
import { useActivityBarStore } from "@/store/activityBar";
import { cn } from "@/utils";

export interface BaseSidebarProps {
  className?: string;
  children?: ReactNode;
}

export const BaseSidebar = ({ className, children }: BaseSidebarProps) => {
  const { data: sideBar } = useGetSidebarPanel();

  return (
    <div
      className={cn(
        "background-(--moss-secondary-background) flex h-full grow flex-col",
        {
          "border-(--moss-border) border-l": sideBar?.position === SIDEBAR_POSITION.LEFT,
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
  const lastRestoredWorkspaceId = useRef<string | undefined>(undefined);

  const lastActiveGroupRef = useRef<string | undefined>(undefined);

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
        lastRestoredWorkspaceId.current = undefined;
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
          <div className="border-t-(--moss-border) z-20 flex w-full justify-end border-t px-1 py-2">
            <WorkspaceModeToggle />
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
          <div className="border-t-(--moss-border) z-20 flex w-full justify-end border-t px-1 py-2">
            <WorkspaceModeToggle />
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
        <div className="border-t-(--moss-border) z-20 flex w-full justify-end border-t px-1 py-2">
          <WorkspaceModeToggle />
        </div>
      )}
    </BaseSidebar>
  );
};

export default Sidebar;
