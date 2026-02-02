import { cn } from "@/utils";
import { useGetLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION } from "@/workbench/domains/layout";

import { ActivityBarButton } from "./components/ActivityBarButton";
import { ActivityBarButtonIndicator } from "./components/ActivityBarButtonIndicator";
import { useMonitorActivityBarFirstItems } from "./hooks/useMonitorActivityBarFirstItems";
import { useSyncedActivityBarFirstItems } from "./hooks/useSyncedActivityBarFirstItems";

export const ActivityBarFirstItems = () => {
  const { data: layout } = useGetLayout();
  const { items } = useSyncedActivityBarFirstItems();

  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;

  useMonitorActivityBarFirstItems();

  return (
    <div
      className={cn("flex", {
        "flex-col gap-3": activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT,
        "flex-row gap-1":
          activityBarPosition === ACTIVITYBAR_POSITION.TOP || activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM,
      })}
    >
      {items.map((item) => {
        const isActive = item.id === layout?.activitybarState.activeContainerId;
        return (
          <div
            key={item.id}
            className={cn("relative flex flex-col", {
              "px-1.5": activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT,
              "py-[3px]":
                activityBarPosition === ACTIVITYBAR_POSITION.TOP || activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM,
            })}
          >
            <ActivityBarButton
              key={item.id}
              id={item.id}
              title={item.title}
              icon={item.icon}
              iconActive={item.iconActive}
              order={item.order}
              isDraggable={true}
            />

            {isActive && layout?.sidebarState.visible && <ActivityBarButtonIndicator />}
          </div>
        );
      })}
    </div>
  );
};
