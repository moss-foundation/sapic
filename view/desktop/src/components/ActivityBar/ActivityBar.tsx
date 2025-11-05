import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useGetSidebarPanel } from "@/hooks/sharedStorage/layout/useGetSidebarPanel";
import { useActivityBarStore } from "@/store/activityBar";
import { cn } from "@/utils";

import { ActivityBarFirstItems } from "./ActivityBarFirstItems";
import { ActivityBarLastItems } from "./ActivityBarLastItems";

export const ActivityBar = () => {
  const { position } = useActivityBarStore();
  const { data: sideBar } = useGetSidebarPanel();

  return (
    <div
      className={cn("background-(--moss-activityBar-background) flex items-center justify-between gap-3", {
        "border-b-(--moss-border) w-full border-b px-1.5": position === ACTIVITYBAR_POSITION.TOP,
        "border-t-(--moss-border) w-full border-t px-1.5": position === ACTIVITYBAR_POSITION.BOTTOM,
        "h-full flex-col py-1.5": position === ACTIVITYBAR_POSITION.DEFAULT,
        "hidden": position === ACTIVITYBAR_POSITION.HIDDEN,

        "border-l-(--moss-border) border-l":
          sideBar?.position === SIDEBAR_POSITION.RIGHT && position === ACTIVITYBAR_POSITION.DEFAULT,
      })}
    >
      <ActivityBarFirstItems />
      <ActivityBarLastItems />
    </div>
  );
};

export default ActivityBar;
