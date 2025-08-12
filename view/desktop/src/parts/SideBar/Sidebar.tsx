import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar } from "@/components";
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
        "background-(--moss-secondary-background) relative flex h-full flex-col",
        {
          "border-l border-(--moss-border-color)": sideBarPosition === SIDEBAR_POSITION.LEFT,
        },
        className
      )}
    >
      {/* <div className="absolute top-0 left-[34px] z-100 h-full w-px bg-red-500" />
      <div className="absolute top-0 left-[10px] z-100 h-full w-px bg-blue-500" />
      <div className="absolute top-0 left-[26px] z-100 h-full w-px bg-blue-500" />

      <div className="absolute top-0 right-2 z-100 h-full w-px bg-red-500" /> */}
      {children}
    </div>
  );
};

export const Sidebar = () => {
  const { data: projectSessionState } = useGetProjectSessionState();
  const { activeWorkspaceId, hasActiveWorkspace, activeWorkspace } = useActiveWorkspace();

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
    if (!activeWorkspaceId || !isFetched || !isSuccess || !workspaceState?.activitybar) return;

    if (lastRestoredWorkspaceId.current === activeWorkspaceId) return;

    // Only restore if we have fresh workspace state for this workspace
    // Add a small delay to ensure workspace switching is complete
    const timeoutId = setTimeout(() => {
      updateFromWorkspaceState(workspaceState.activitybar!);
      lastRestoredWorkspaceId.current = activeWorkspaceId;
    }, 100);

    return () => clearTimeout(timeoutId);
  }, [activeWorkspaceId, workspaceState?.activitybar, isFetched, isSuccess, updateFromWorkspaceState]);

  const { position } = useActivityBarStore();

  const activeItem = useActivityBarStore((state) => state.getActiveItem());

  const activeGroupId = activeItem?.id || "default";

  // Content based on workspace status
  // Pass the workspace name and groupId to the SidebarWorkspaceContent component
  const sidebarContent = hasActiveWorkspace ? (
    <SidebarWorkspaceContent workspaceName={activeWorkspace!.name} groupId={activeGroupId} />
  ) : (
    <EmptyWorkspace inSidebar={true} />
  );

  if (position === ACTIVITYBAR_POSITION.TOP) {
    return (
      <BaseSidebar>
        <ActivityBar />
        {sidebarContent}
      </BaseSidebar>
    );
  }

  if (position === ACTIVITYBAR_POSITION.BOTTOM) {
    return (
      <BaseSidebar className="relative">
        <div className="flex-1 overflow-auto">{sidebarContent}</div>
        <ActivityBar />
      </BaseSidebar>
    );
  }

  return <BaseSidebar>{sidebarContent}</BaseSidebar>;
};

export default Sidebar;
