import { HorizontalActivityBar } from "@/parts/ActivityBar/HorizontalActivityBar";
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
  const isActivityBarInDefaultPosition = activityBarState?.position === "default";

  // If sidebar is hidden, don't render anything in the Sidebar component
  if (!isVisible) {
    return null;
  }

  const Content = () => (
    <>
      <SidebarHeader title={activeGroupTitle} />
      {isCollectionsActive ? <CollectionTreeView /> : <ViewContainer groupId={activeGroupId} />}
    </>
  );

  if (isActivityBarInDefaultPosition) {
    return (
      <div className={cn("background-(--moss-sideBar-background) flex h-full flex-col")}>
        <Content />
      </div>
    );
  }

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
        <HorizontalActivityBar position="top" />
        <Content />
      </div>
    );
  }

  if (effectivePosition === "bottom") {
    return (
      <div className={cn("background-(--moss-sideBar-background) flex h-full flex-col")}>
        <Content />
        <HorizontalActivityBar position="bottom" />
      </div>
    );
  }

  // These cases should never happen due to isActivityBarInDefaultPosition check,
  // but keeping them for completeness
  if (effectivePosition === "left") {
    return (
      <div className={cn("background-(--moss-sideBar-background) flex h-full flex-col")}>
        <Content />
      </div>
    );
  }

  if (effectivePosition === "right") {
    return (
      <div className={cn("background-(--moss-sideBar-background) flex h-full flex-col")}>
        <Content />
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
