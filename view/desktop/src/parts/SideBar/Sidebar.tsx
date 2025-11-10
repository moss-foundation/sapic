import { ReactNode } from "react";

import { ActivityBar } from "@/components";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import WorkspaceModeToggle from "@/components/WorkspaceModeToggle";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutStates";
import { useActiveWorkspace, useDescribeApp } from "@/hooks";
import { useGetLayout } from "@/hooks/sharedStorage/layout/useGetLayout";
import { SidebarWorkspaceContent } from "@/parts/SideBar/SidebarWorkspaceContent";
import { cn } from "@/utils";

export interface BaseSidebarProps {
  className?: string;
  children?: ReactNode;
}

export const BaseSidebar = ({ className, children }: BaseSidebarProps) => {
  const { data: appState } = useDescribeApp();
  const sideBarPosition = appState?.configuration.contents.sideBarPosition || SIDEBAR_POSITION.LEFT;

  return (
    <div
      className={cn(
        "background-(--moss-secondary-background) flex h-full grow flex-col",
        {
          "border-(--moss-border) border-l": sideBarPosition === SIDEBAR_POSITION.LEFT,
        },
        className
      )}
    >
      {children}
    </div>
  );
};

export const Sidebar = () => {
  const { data: appState } = useDescribeApp();
  const { hasActiveWorkspace } = useActiveWorkspace();
  const { data: layout } = useGetLayout();

  const activityBarPosition = appState?.configuration.contents.activityBarPosition || ACTIVITYBAR_POSITION.DEFAULT;
  const activeGroupId = layout?.activitybarState.activeContainerId;

  const sidebarContent = hasActiveWorkspace ? (
    <SidebarWorkspaceContent groupId={activeGroupId} />
  ) : (
    <EmptyWorkspace inSidebar={true} />
  );

  if (activityBarPosition === ACTIVITYBAR_POSITION.TOP) {
    return (
      <BaseSidebar>
        <ActivityBar />
        <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
        {hasActiveWorkspace && (
          <div className="border-t-(--moss-border) z-20 flex w-full justify-end border-t px-1 py-2">
            <WorkspaceModeToggle />
          </div>
        )}
      </BaseSidebar>
    );
  }

  if (activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM) {
    return (
      <BaseSidebar>
        <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
        {hasActiveWorkspace && (
          <div className="border-t-(--moss-border) z-20 flex w-full justify-end border-t px-1 py-2">
            <WorkspaceModeToggle />
          </div>
        )}
        <ActivityBar />
      </BaseSidebar>
    );
  }

  return (
    <BaseSidebar>
      <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
      {hasActiveWorkspace && (
        <div className="border-t-(--moss-border) z-20 flex w-full justify-end border-t px-1 py-2">
          <WorkspaceModeToggle />
        </div>
      )}
    </BaseSidebar>
  );
};

export default Sidebar;
