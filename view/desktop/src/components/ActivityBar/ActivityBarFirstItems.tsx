import { useEffect } from "react";

import { ACTIVITYBAR_POSITION } from "@/constants/layoutStates";
import { useGetLayout } from "@/hooks/sharedStorage/layout/useGetLayout";
import { ActivityBarItemProps, useActivityBarStore } from "@/store/activityBar";
import { cn } from "@/utils";
import { swapListById } from "@/utils/swapListById";
import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ActivityBarButton } from "./ActivityBarButton";
import { ActivityBarButtonIndicator } from "./ActivityBarButtonIndicator";

export const ActivityBarFirstItems = () => {
  const { items, position, setItems } = useActivityBarStore();
  const { data: layout } = useGetLayout();

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return source.data.type === "ActivityBarButton";
      },
      onDrop({ location, source }) {
        const target = location.current.dropTargets[0];
        if (!target) return;

        const sourceData = source.data as { data: ActivityBarItemProps };
        const targetData = target.data as { data: ActivityBarItemProps };
        const edge = extractClosestEdge(targetData);

        if (!sourceData || !targetData || !sourceData.data || !targetData.data) return;

        const updatedItems = swapListById(sourceData.data.id, targetData.data.id, items, edge);

        if (!updatedItems) return;

        setItems(updatedItems);
      },
    });
  }, [items, setItems]);

  return (
    <div
      className={cn("flex", {
        "flex-col gap-3": position === ACTIVITYBAR_POSITION.DEFAULT,
        "flex-row gap-1": position === ACTIVITYBAR_POSITION.TOP || position === ACTIVITYBAR_POSITION.BOTTOM,
      })}
    >
      {items
        .filter((item) => item.isVisible !== false)
        .map((item) => {
          const isActive = item.id === layout?.activitybarState.activeContainerId;
          return (
            <div
              key={item.id}
              className={cn("relative flex flex-col", {
                "px-1.5": position === ACTIVITYBAR_POSITION.DEFAULT,
                "py-[3px]": position === ACTIVITYBAR_POSITION.TOP || position === ACTIVITYBAR_POSITION.BOTTOM,
              })}
            >
              <ActivityBarButton key={item.id} {...item} />

              {isActive && layout?.sidebarState.visible && <ActivityBarButtonIndicator />}
            </div>
          );
        })}
    </div>
  );
};
