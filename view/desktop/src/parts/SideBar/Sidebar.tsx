import { ActivityBar } from "@/components/ActivityBar";
import { ViewContainer } from "@/components/ViewContainer";
import { CollectionTreeView } from "@/components/CollectionTreeView";
import { useGetActivityBarState } from "@/hooks/useActivityBarState";
import { useGetAppLayoutState } from "@/hooks/useAppLayoutState";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useGetViewGroups } from "@/hooks/useViewGroups";
import { cn } from "@/utils";
import { useEffect, useRef } from "react";

import SidebarHeader from "./SidebarHeader";

export const Sidebar = () => {
  const { data: activityBarState } = useGetActivityBarState();
  const { data: appLayoutState } = useGetAppLayoutState();
  const { data: viewGroups } = useGetViewGroups();
  const { data: projectSessionState } = useGetProjectSessionState();

  // Use a ref to preserve the lastActiveGroup when sidebar position changes
  const lastActiveGroupRef = useRef<string | null>(null);

  useEffect(() => {
    if (projectSessionState?.lastActiveGroup) {
      lastActiveGroupRef.current = projectSessionState.lastActiveGroup;
    }
  }, [projectSessionState?.lastActiveGroup]);

  // When appLayoutState changes, ensure we're using the reference to preserve the active state
  const activeGroupId = projectSessionState?.lastActiveGroup || lastActiveGroupRef.current || "";

  const activeGroup = viewGroups?.viewGroups?.find((group) => group.id === activeGroupId);
  const activeGroupTitle = activeGroup?.title || "Launchpad";

  const isCollectionsActive = activeGroupId === "collections.groupId";

  const getEffectivePosition = () => {
    if (!activityBarState || !appLayoutState) return "left";

    if (activityBarState.position !== "default") {
      return activityBarState.position;
    }

    return appLayoutState.activeSidebar === "right" ? "right" : "left";
  };

  const effectivePosition = getEffectivePosition();

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
