import { ReactNode, useEffect, useRef } from "react";

import { CollectionTreeView } from "@/components/CollectionTreeView";
import { ViewContainer } from "@/components/ViewContainer";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";

import { ActivityBar } from "../ActivityBar";
import SidebarHeader from "./SidebarHeader";

export interface BaseSidebarProps {
  className?: string;
  children?: ReactNode;
}

export const BaseSidebar = ({ className, children }: BaseSidebarProps) => {
  const { primarySideBarPosition } = useAppResizableLayoutStore();

  return (
    <div
      className={cn(
        "background-(--moss-secondary-background) flex h-full flex-col",
        {
          "border-r border-(--moss-border-color)": primarySideBarPosition === "left",
          "border-l border-(--moss-border-color)": primarySideBarPosition === "right",
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

  const activeGroupId = activeItem?.id;
  const isCollectionsActive = activeGroupId === "collections.groupId";
  const activeGroupTitle = activeItem?.title || "Launchpad";

  if (position === "top") {
    return (
      <BaseSidebar>
        <ActivityBar />
        <div className="flex grow flex-col">
          <SidebarHeader title={activeGroupTitle} />
          {isCollectionsActive ? <CollectionTreeView /> : <ViewContainer groupId={activeGroupId} />}
        </div>
      </BaseSidebar>
    );
  }

  if (position === "bottom") {
    return (
      <BaseSidebar>
        <div className="flex grow flex-col">
          <SidebarHeader title={activeGroupTitle} />
          {isCollectionsActive ? <CollectionTreeView /> : <ViewContainer groupId={activeGroupId} />}
        </div>
        <ActivityBar />
      </BaseSidebar>
    );
  }

  return (
    <BaseSidebar>
      <div className="flex grow flex-col">
        <SidebarHeader title={activeGroupTitle} />
        {isCollectionsActive ? <CollectionTreeView /> : <ViewContainer groupId={activeGroupId} />}
      </div>
    </BaseSidebar>
  );
};

export default Sidebar;
