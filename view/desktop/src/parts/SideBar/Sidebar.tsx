import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar } from "@/components";
import { SidebarWorkspaceContent } from "@/components/SidebarWorkspaceContent";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";
import { useActiveWorkspace } from "@/hooks";

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
  const workspace = useActiveWorkspace();
  const hasWorkspace = !!workspace;

  const lastActiveGroupRef = useRef<string | null>(null);

  useEffect(() => {
    if (projectSessionState?.lastActiveGroup) {
      lastActiveGroupRef.current = projectSessionState.lastActiveGroup;
    }
  }, [projectSessionState?.lastActiveGroup]);

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
