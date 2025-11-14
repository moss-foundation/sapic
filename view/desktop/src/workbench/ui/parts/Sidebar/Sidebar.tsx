import { ReactNode } from "react";

import { ActivityBar } from "@/components";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import WorkspaceModeToggle from "@/components/WorkspaceModeToggle";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layout";
import { useActiveWorkspace, useDescribeApp } from "@/hooks";
import { cn } from "@/utils";
import { SidebarWorkspaceContent } from "@/workbench/ui/parts/Sidebar/SidebarWorkspaceContent";

export interface BaseSidebarProps {
  className?: string;
  children?: ReactNode;
}

const BaseSidebar = ({ className, children }: BaseSidebarProps) => {
  const { data: appState } = useDescribeApp();
  //TODO later we should handle the JsonValue differently
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

  const activityBarPosition = appState?.configuration.contents.activityBarPosition || ACTIVITYBAR_POSITION.DEFAULT;

  const sidebarContent = hasActiveWorkspace ? <SidebarWorkspaceContent /> : <EmptyWorkspace inSidebar={true} />;

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
