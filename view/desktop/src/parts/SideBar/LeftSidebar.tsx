import { useGetActivityBarState } from "@/hooks/useGetActivityBarState";
import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { HorizontalActivityBar } from "@/parts/ActivityBar/HorizontalActivityBar";
import { ViewContainer } from "@/components/ViewContainer";
import { CollectionTreeView } from "@/components/CollectionTreeView";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useGetViewGroups } from "@/hooks/useGetViewGroups";
import { useEffect, useRef } from "react";

import SidebarHeader from "./SidebarHeader";
import { BaseSidebar } from "./Sidebar";

interface LeftSidebarProps {
  isResizing?: boolean;
}

export const LeftSidebar = ({ isResizing = false }: LeftSidebarProps) => {
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
  const isActivityBarInDefaultPosition = activityBarState?.position === "default";

  const Content = () => (
    <>
      <SidebarHeader title={activeGroupTitle} />
      {isCollectionsActive ? <CollectionTreeView /> : <ViewContainer groupId={activeGroupId} />}
    </>
  );

  // Using the BaseSidebar component with position="left"
  if (isActivityBarInDefaultPosition) {
    return (
      <BaseSidebar isResizing={isResizing} position="left">
        <Content />
      </BaseSidebar>
    );
  }

  if (effectivePosition === "hidden") {
    return (
      <BaseSidebar isResizing={isResizing} position="left">
        <Content />
      </BaseSidebar>
    );
  }

  if (effectivePosition === "top") {
    return (
      <BaseSidebar isResizing={isResizing} position="left">
        <HorizontalActivityBar position="top" />
        <Content />
      </BaseSidebar>
    );
  }

  if (effectivePosition === "bottom") {
    return (
      <BaseSidebar isResizing={isResizing} position="left">
        <Content />
        <HorizontalActivityBar position="bottom" />
      </BaseSidebar>
    );
  }

  // Default case
  return (
    <BaseSidebar isResizing={isResizing} position="left">
      <Content />
    </BaseSidebar>
  );
};

export default LeftSidebar;
