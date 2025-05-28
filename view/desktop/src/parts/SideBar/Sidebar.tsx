import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar } from "@/components";
import { Workspace } from "@/components/Workspace";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";
import { useDescribeAppState, useWorkspaceMapping } from "@/hooks";

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
          "border-l border-(--moss-border-color)": sideBarPosition === "left",
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
  const { data: appState } = useDescribeAppState();
  const { getNameById } = useWorkspaceMapping();

  const lastActiveGroupRef = useRef<string | null>(null);

  useEffect(() => {
    if (projectSessionState?.lastActiveGroup) {
      lastActiveGroupRef.current = projectSessionState.lastActiveGroup;
    }
  }, [projectSessionState?.lastActiveGroup]);

  const { position } = useActivityBarStore();

  const activeItem = useActivityBarStore((state) => state.getActiveItem());

  const activeGroupId = activeItem?.id || "empty";
  const activeGroupTitle = activeItem?.title || "Launchpad";

  // Get workspace name from appState
  const currentWorkspaceId = appState?.lastWorkspace;
  const currentWorkspaceName = currentWorkspaceId ? getNameById(currentWorkspaceId) : null;
  const hasWorkspace = !!currentWorkspaceName;

  // Content based on workspace status
  // Pass the workspace name to the Workspace component
  const sidebarContent = hasWorkspace ? (
    <Workspace groupId={activeGroupId} workspaceName={currentWorkspaceName} />
  ) : (
    <EmptyWorkspace inSidebar={true} />
  );

  if (position === "top") {
    return (
      <BaseSidebar>
        <ActivityBar />
        <SidebarHeader title={activeGroupTitle} />
        {sidebarContent}
      </BaseSidebar>
    );
  }

  if (position === "bottom") {
    return (
      <BaseSidebar>
        <SidebarHeader title={activeGroupTitle} />
        {sidebarContent}
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
