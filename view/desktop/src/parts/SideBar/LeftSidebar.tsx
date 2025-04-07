import { useEffect, useRef } from "react";

import { CollectionTreeView } from "@/components/CollectionTreeView";
import { ViewContainer } from "@/components/ViewContainer";
import { useGetActivityBarState } from "@/hooks/useGetActivityBarState";
import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { useGetViewGroups } from "@/hooks/useGetViewGroups";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { HorizontalActivityBar } from "@/parts/ActivityBar/HorizontalActivityBar";

import { Sidebar } from "./Sidebar";
import SidebarHeader from "./SidebarHeader";

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
      <Sidebar isResizing={isResizing} position="left">
        <Content />
      </Sidebar>
    );
  }

  if (effectivePosition === "hidden") {
    return (
      <Sidebar isResizing={isResizing} position="left">
        <Content />
      </Sidebar>
    );
  }

  if (effectivePosition === "top") {
    return (
      <Sidebar isResizing={isResizing} position="left">
        <HorizontalActivityBar position="top" />
        <Content />
      </Sidebar>
    );
  }

  if (effectivePosition === "bottom") {
    return (
      <Sidebar isResizing={isResizing} position="left">
        <Content />
        <HorizontalActivityBar position="bottom" />
      </Sidebar>
    );
  }

  // Default case
  return (
    <Sidebar isResizing={isResizing} position="left">
      <Content />
    </Sidebar>
  );
};

export default LeftSidebar;
