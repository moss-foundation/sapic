import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar } from "@/components";
import { SidebarWorkspaceContent } from "@/components/SidebarWorkspaceContent";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";
import { useActiveWorkspace } from "@/hooks";
import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";

import SidebarHeader from "./SidebarHeader";

export interface BaseSidebarProps {
  className?: string;
  children?: ReactNode;
}

export const BaseSidebar = ({ className, children }: BaseSidebarProps) => {
  const { sideBarPosition } = useAppResizableLayoutStore();

  return (
    <div
      className={cn(
        "background-(--moss-secondary-background) flex h-full flex-col",
        {
          "border-l border-(--moss-border-color)": sideBarPosition === "LEFT",
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

  const { data: workspaceState } = useDescribeWorkspaceState();
  const { updateFromWorkspaceState } = useActivityBarStore();
  const hasRestoredActivityBarState = useRef(false);

  const lastActiveGroupRef = useRef<string | null>(null);

  useEffect(() => {
    if (projectSessionState?.lastActiveGroup) {
      lastActiveGroupRef.current = projectSessionState.lastActiveGroup;
    }
  }, [projectSessionState?.lastActiveGroup]);

  // Restore activity bar state from workspace state (only once)
  useEffect(() => {
    if (workspaceState?.activitybar && !hasRestoredActivityBarState.current) {
      updateFromWorkspaceState(workspaceState.activitybar);
      hasRestoredActivityBarState.current = true;
    }
  }, [workspaceState?.activitybar, updateFromWorkspaceState]);

  // Reset the restoration flag when workspace changes
  useEffect(() => {
    hasRestoredActivityBarState.current = false;
  }, [workspace?.id]);

  const { position } = useActivityBarStore();

  const activeItem = useActivityBarStore((state) => state.getActiveItem());

  const activeGroupTitle = activeItem?.title || "Launchpad";
  const activeGroupId = activeItem?.id || "default";

  // Content based on workspace status
  // Pass the workspace name and groupId to the SidebarWorkspaceContent component
  const sidebarContent = hasWorkspace ? (
    <SidebarWorkspaceContent workspaceName={workspace!.displayName} groupId={activeGroupId} />
  ) : (
    <EmptyWorkspace inSidebar={true} />
  );

  if (position === "TOP") {
    return (
      <BaseSidebar>
        <ActivityBar />
        <SidebarHeader title={activeGroupTitle} />
        {sidebarContent}
      </BaseSidebar>
    );
  }

  if (position === "BOTTOM") {
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
