import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layout";
import { useDescribeApp } from "@/hooks/app/useDescribeApp";
import { cn } from "@/utils";

import { ActivityBarFirstItems } from "./ActivityBarFirstItems";
import { ActivityBarLastItems } from "./ActivityBarLastItems";

export const ActivityBar = () => {
  const { data: appState } = useDescribeApp();

  //TODO later we should handle the JsonValue differently
  const activityBarPosition = appState?.configuration.contents.activityBarPosition || ACTIVITYBAR_POSITION.DEFAULT;
  const sideBarPosition = appState?.configuration.contents.sideBarPosition || SIDEBAR_POSITION.LEFT;

  return (
    <div
      className={cn("background-(--moss-activityBar-background) flex items-center justify-between gap-3", {
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
