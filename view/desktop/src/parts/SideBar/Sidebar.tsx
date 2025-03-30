import { ActivityBar } from "@/components/ActivityBar";
import { ViewContainer } from "@/components/ViewContainer";
import { CollectionTreeView } from "@/components/CollectionTreeView";
import { useGetActivityBarState } from "@/hooks/useGetActivityBarState";
import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useGetViewGroups } from "@/hooks/useGetViewGroups";
import { cn } from "@/utils";
import { useEffect, useRef } from "react";

import SidebarHeader from "./SidebarHeader";

export const Sidebar = () => {
  const { data: activityBarState } = useGetActivityBarState();
  const { data: appLayoutState } = useGetAppLayoutState();
  const { data: viewGroups } = useGetViewGroups();
  const { data: projectSessionState } = useGetProjectSessionState();

  const lastActiveGroupRef = useRef<string | null>(null);

  useEffect(() => {
    if (projectSessionState?.lastActiveGroup) {
      lastActiveGroupRef.current = projectSessionState.lastActiveGroup;
    }
  }, [projectSessionState?.lastActiveGroup]);

  const activeGroupId = projectSessionState?.lastActiveGroup || lastActiveGroupRef.current || "";

  const activeGroup = viewGroups?.viewGroups?.find((group) => group.id === activeGroupId);
  const activeGroupTitle = activeGroup?.title || "Launchpad";

  const isCollectionsActive = activeGroupId === "collections.groupId";

  const getEffectivePosition = () => {
    if (!activityBarState || !appLayoutState) return "left";

    if (activityBarState.position !== "default") {
      return activityBarState.position;
    }

    return appLayoutState.sidebarSetting || "left";
  };

  const effectivePosition = getEffectivePosition();
  const isVisible = appLayoutState?.activeSidebar !== "none";

  if (!isVisible) {
    return null;
  }

  const Content = () => (
    <>
      <SidebarHeader title={activeGroupTitle} />
      {isCollectionsActive ? <CollectionTreeView /> : <ViewContainer groupId={activeGroupId} />}
    </>
  );

  if (effectivePosition === "hidden") {
    return (
      <div className={cn("background-(--moss-sideBar-background) flex h-full flex-col")}>
        <Content />
      </div>
    );
  }

  if (effectivePosition === "top") {
    return (
      <div className={cn("background-(--moss-sideBar-background) flex h-full flex-col")}>
        <ActivityBar />
        <Content />
      </div>
    );
  }

  if (effectivePosition === "bottom") {
    return (
      <div className={cn("background-(--moss-sideBar-background) flex h-full flex-col")}>
        <Content />
        <ActivityBar />
      </div>
    );
  }

  if (effectivePosition === "left") {
    return (
      <div className={cn("background-(--moss-sideBar-background) flex h-full")}>
        <ActivityBar />
        <div className="w-full">
          <Content />
        </div>
      </div>
    );
  }

  if (effectivePosition === "right") {
    return (
      <div className={cn("background-(--moss-sideBar-background) flex h-full")}>
        <div className="w-[calc(100%-41px)]">
          <Content />
        </div>
        <ActivityBar />
      </div>
    );
  }

  return (
    <div className={cn("background-(--moss-sideBar-background) flex h-full flex-col")}>
      <Content />
    </div>
  );
};

export default Sidebar;
