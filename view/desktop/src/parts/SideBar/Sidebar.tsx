import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar } from "@/components";
import { ViewContainer } from "@/components/ViewContainer";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";

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

  if (position === "top") {
    return (
      <BaseSidebar>
        <ActivityBar />
        <SidebarHeader title={activeGroupTitle} />
        <ViewContainer groupId={activeGroupId} />
      </BaseSidebar>
    );
  }

  if (position === "bottom") {
    return (
      <BaseSidebar>
        <SidebarHeader title={activeGroupTitle} />
        <ViewContainer groupId={activeGroupId} />
        <ActivityBar />
      </BaseSidebar>
    );
  }

  return (
    <BaseSidebar>
      <SidebarHeader title={activeGroupTitle} />
      <ViewContainer groupId={activeGroupId} />
    </BaseSidebar>
  );
};

export default Sidebar;
