import { cn } from "@/utils";
import { useGetLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/workbench/domains/layout";

import { ActivityBarFirstItems } from "./ActivityBarFirstItems";
import { ActivityBarLastItems } from "./ActivityBarLastItems";

export const ActivityBar = () => {
  const { data: layout } = useGetLayout();

  //TODO later we should handle the JsonValue differently
  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;
  const sideBarPosition = layout?.sidebarState.position || SIDEBAR_POSITION.LEFT;

  return (
    <div
      className={cn("background-(--moss-primary-background) flex items-center justify-between gap-3", {
        "border-b-(--moss-border) w-full border-b px-1.5": activityBarPosition === ACTIVITYBAR_POSITION.TOP,
        "border-t-(--moss-border) w-full border-t px-1.5": activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM,
        "h-full flex-col py-1.5": activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT,
        "hidden": activityBarPosition === ACTIVITYBAR_POSITION.HIDDEN,

        "border-l-(--moss-border) border-l":
          sideBarPosition === SIDEBAR_POSITION.RIGHT && activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT,
      })}
    >
      <ActivityBarFirstItems />
      <ActivityBarLastItems />
    </div>
  );
};

export default ActivityBar;
