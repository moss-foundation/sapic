import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";

import { ActivityBarFirstItems } from "./ActivityBarFirstItems";
import { ActivityBarLastItems } from "./ActivityBarLastItems";

export const ActivityBar = () => {
  const { position } = useActivityBarStore();
  const sideBarPosition = useAppResizableLayoutStore((state) => state.sideBarPosition);

  return (
    <div
      className={cn("background-(--moss-secondary-background) flex items-center justify-between gap-3", {
        "w-full border-b border-b-(--moss-border-color) px-1.5": position === ACTIVITYBAR_POSITION.TOP,
        "w-full border-t border-t-(--moss-border-color) px-1.5": position === ACTIVITYBAR_POSITION.BOTTOM,
        "h-full flex-col py-1.5": position === ACTIVITYBAR_POSITION.DEFAULT,
        "hidden": position === ACTIVITYBAR_POSITION.HIDDEN,

        "border-l border-l-(--moss-border-color)":
          sideBarPosition === SIDEBAR_POSITION.RIGHT && position === ACTIVITYBAR_POSITION.DEFAULT,
      })}
    >
      <ActivityBarFirstItems />
      <ActivityBarLastItems />
    </div>
  );
};

export default ActivityBar;
