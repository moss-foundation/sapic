import { ReactNode } from "react";

import { cn } from "@/utils";
import { useGetLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/workbench/domains/layout";
import { ActivityBar } from "@/workbench/ui/components";
import WorkspaceModeToggle from "@/workbench/ui/components/WorkspaceModeToggle";
import { SidebarWorkspaceContent } from "@/workbench/ui/parts/Sidebar/SidebarWorkspaceContent";

export interface BaseSidebarProps {
  className?: string;
  children?: ReactNode;
}

const BaseSidebar = ({ className, children }: BaseSidebarProps) => {
  const { data: layout } = useGetLayout();
  //TODO later we should handle the JsonValue differently
  const sideBarPosition = layout?.sidebarState.position || SIDEBAR_POSITION.LEFT;

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
  const { data: layout } = useGetLayout();

  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;

  const sidebarContent = <SidebarWorkspaceContent />;

  if (activityBarPosition === ACTIVITYBAR_POSITION.TOP) {
    return (
      <BaseSidebar>
        <ActivityBar />
        <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>

        <div className="border-t-(--moss-border) z-20 flex w-full justify-end border-t px-1 py-2">
          <WorkspaceModeToggle />
        </div>
      </BaseSidebar>
    );
  }

  if (activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM) {
    return (
      <BaseSidebar>
        <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>

        <div className="border-t-(--moss-border) z-20 flex w-full justify-end border-t px-1 py-2">
          <WorkspaceModeToggle />
        </div>

        <ActivityBar />
      </BaseSidebar>
    );
  }

  return (
    <BaseSidebar>
      <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>

      <div className="border-t-(--moss-border) z-20 flex w-full justify-end border-t px-1 py-2">
        <WorkspaceModeToggle />
      </div>
    </BaseSidebar>
  );
};
