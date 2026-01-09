import { ReactNode } from "react";

import { cn } from "@/utils";
import { useGetLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/workbench/domains/layout";
import { ActivityBar } from "@/workbench/ui/components";
import { SidebarContent } from "@/workbench/ui/parts/Sidebar/SidebarContent";

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
        "background-(--moss-primary-background) flex h-full grow flex-col",
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

  const sidebarContent = <SidebarContent />;

  if (activityBarPosition === ACTIVITYBAR_POSITION.TOP) {
    return (
      <BaseSidebar>
        <ActivityBar />
        <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
      </BaseSidebar>
    );
  }

  if (activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM) {
    return (
      <BaseSidebar>
        <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
        <ActivityBar />
      </BaseSidebar>
    );
  }

  return (
    <BaseSidebar>
      <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
    </BaseSidebar>
  );
};
