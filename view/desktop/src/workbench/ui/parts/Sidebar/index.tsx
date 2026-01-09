import { cn } from "@/utils";
import { useGetLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/workbench/domains/layout";
import { ActivityBar } from "@/workbench/ui/components";
import { SidebarContent } from "@/workbench/ui/parts/Sidebar/SidebarContent";

export const Sidebar = () => {
  const { data: layout } = useGetLayout();

  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;
  const sideBarPosition = layout?.sidebarState.position || SIDEBAR_POSITION.LEFT;
  const sidebarContent = <SidebarContent />;

  const baseClassName = cn("background-(--moss-primary-background) flex h-full grow flex-col", {
    "border-(--moss-border) border-l": sideBarPosition === SIDEBAR_POSITION.LEFT,
  });

  if (activityBarPosition === ACTIVITYBAR_POSITION.TOP) {
    return (
      <div className={baseClassName}>
        <ActivityBar />
        <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
      </div>
    );
  }

  if (activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM) {
    return (
      <div className={baseClassName}>
        <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
        <ActivityBar />
      </div>
    );
  }

  return (
    <div className={baseClassName}>
      <div className="min-h-0 flex-1 overflow-hidden">{sidebarContent}</div>
    </div>
  );
};
