import { ReactNode, useEffect, useRef } from "react";

import { CollectionTreeView } from "@/components/CollectionTreeView";
import { ViewContainer } from "@/components/ViewContainer";
import { useGetActivityBarState } from "@/hooks/useGetActivityBarState";
import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { useGetViewGroups } from "@/hooks/useGetViewGroups";
import { useGetProjectSessionState } from "@/hooks/useProjectSession";
import { HorizontalActivityBar } from "@/parts/ActivityBar/HorizontalActivityBar";
import { cn } from "@/utils";

import SidebarHeader from "./SidebarHeader";

export interface BaseSidebarProps {
  isResizing?: boolean;
  position?: "left" | "right";
  className?: string;
  children?: ReactNode;
}

export const BaseSidebar = ({ isResizing = false, position = "left", className, children }: BaseSidebarProps) => {
  const { data: appLayoutState } = useGetAppLayoutState();

  const sidebarSetting = appLayoutState?.sidebarSetting || "left";
  const activeSidebar = appLayoutState?.activeSidebar || "none";
  const isVisible =
    activeSidebar !== "none" &&
    ((position === "left" && sidebarSetting === "left" && activeSidebar === "left") ||
      (position === "right" && sidebarSetting === "right" && activeSidebar === "right"));

  const sidebarClasses = cn(
    "background-(--moss-secondary-background)  flex h-full flex-col",
    // Only apply transitions when not actively resizing
    !isResizing && "transition-[width] duration-200",
    !isVisible && "w-0 overflow-hidden",
    {
      "border-r border-(--moss-border-color)": position === "left",
      "border-l border-(--moss-border-color)": position === "right",
    },
    className
  );

  return <div className={sidebarClasses}>{children}</div>;
};

export const Sidebar = ({ isResizing = false }: { isResizing?: boolean }) => {
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

  if (isActivityBarInDefaultPosition) {
    return (
      <BaseSidebar isResizing={isResizing} position={appLayoutState?.sidebarSetting as "left" | "right"}>
        <Content />
      </BaseSidebar>
    );
  }

  if (effectivePosition === "hidden") {
    return (
      <BaseSidebar isResizing={isResizing} position={appLayoutState?.sidebarSetting as "left" | "right"}>
        <Content />
      </BaseSidebar>
    );
  }

  if (effectivePosition === "top") {
    return (
      <BaseSidebar isResizing={isResizing} position={appLayoutState?.sidebarSetting as "left" | "right"}>
        <HorizontalActivityBar position="top" />
        <Content />
      </BaseSidebar>
    );
  }

  if (effectivePosition === "bottom") {
    return (
      <BaseSidebar isResizing={isResizing} position={appLayoutState?.sidebarSetting as "left" | "right"}>
        <Content />
        <HorizontalActivityBar position="bottom" />
      </BaseSidebar>
    );
  }

  return (
    <BaseSidebar isResizing={isResizing} position={appLayoutState?.sidebarSetting as "left" | "right"}>
      <Content />
    </BaseSidebar>
  );
};

export default Sidebar;
