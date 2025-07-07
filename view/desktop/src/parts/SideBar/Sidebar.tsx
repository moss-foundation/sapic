import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar } from "@/components";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { SidebarWorkspaceContent } from "@/components/SidebarWorkspaceContent";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";

import SidebarHeader from "./SidebarHeader";

export interface BaseSidebarProps {
  className?: string;
  children?: ReactNode;
}

export const BaseSidebar = ({ className, children }: BaseSidebarProps) => {
  const sideBarPosition = useAppResizableLayoutStore((state) => state.sideBarPosition);

  return (
    <div
      className={cn(
        "background-(--moss-secondary-background) flex h-full flex-col",
        {
          "border-l border-(--moss-border-color)": sideBarPosition === SIDEBAR_POSITION.LEFT,
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
  const workspace = useActiveWorkspace();
  const hasWorkspace = !!workspace;
  const workspaceId = workspace?.id || null;

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
    if (lastRestoredWorkspaceId.current !== workspaceId) {
      // Reset to default state before loading new workspace state
      if (workspaceId) {
        // Will be restored from workspace state
        resetToDefaults();
        lastRestoredWorkspaceId.current = null;
      } else {
        // Reset to default when no workspace
        resetToDefaults();
        lastRestoredWorkspaceId.current = workspaceId;
      }
    }
  }, [workspaceId, resetToDefaults]);

  // Restore activity bar state from workspace state
  useEffect(() => {
    if (!workspaceId || !isFetched || !isSuccess || !workspaceState?.activitybar) return;

    if (lastRestoredWorkspaceId.current === workspaceId) return;

    // Only restore if we have fresh workspace state for this workspace
    // Add a small delay to ensure workspace switching is complete
    const timeoutId = setTimeout(() => {
      updateFromWorkspaceState(workspaceState.activitybar!);
      lastRestoredWorkspaceId.current = workspaceId;
    }, 100);

    return () => clearTimeout(timeoutId);
  }, [workspaceId, workspaceState?.activitybar, isFetched, isSuccess, updateFromWorkspaceState]);

  const { position } = useActivityBarStore();

  const activeItem = useActivityBarStore((state) => state.getActiveItem());

  const activeGroupTitle = activeItem?.title || "Launchpad";
  const activeGroupId = activeItem?.id || "default";

  // Content based on workspace status
  // Pass the workspace name and groupId to the SidebarWorkspaceContent component
  const sidebarContent = hasWorkspace ? (
    <SidebarWorkspaceContent workspaceName={workspace!.name} groupId={activeGroupId} />
  ) : (
    <EmptyWorkspace inSidebar={true} />
  );

  if (position === ACTIVITYBAR_POSITION.TOP) {
    return (
      <BaseSidebar>
        <ActivityBar />
        <SidebarHeader title={activeGroupTitle} />
        {sidebarContent}
      </BaseSidebar>
    );
  }

  if (position === ACTIVITYBAR_POSITION.BOTTOM) {
    return (
      <BaseSidebar className="relative">
        <SidebarHeader title={activeGroupTitle} />
        <div className="flex-1 overflow-auto">{sidebarContent}</div>
        <ActivityBar />
      </BaseSidebar>
    );
  }

  return (
    <BaseSidebar>
      <SidebarHeader title={activeGroupTitle} />
      {sidebarContent}
    </BaseSidebar>
  );
};

export default Sidebar;
