import { useEffect } from "react";

import { ACTIVITYBAR_POSITION } from "@/constants/layout";
import { useDescribeApp } from "@/hooks";
import { useGetLayout } from "@/hooks/workbench/layout/useGetLayout";
import { cn } from "@/utils";
import { ActivityBarItemProps, useActivityBarStore } from "@/workbench/store/activityBar";
import { swapListById } from "@/workbench/utils/swapListById";
import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ActivityBarButton } from "./ActivityBarButton";
import { ActivityBarButtonIndicator } from "./ActivityBarButtonIndicator";

export const ActivityBarFirstItems = () => {
  const { data: appState } = useDescribeApp();
  const { data: layout } = useGetLayout();
  const { items, setItems } = useActivityBarStore();

  const activityBarPosition = appState?.configuration.contents.activityBarPosition || ACTIVITYBAR_POSITION.DEFAULT;

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
        "flex-col gap-3": activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT,
        "flex-row gap-1":
          activityBarPosition === ACTIVITYBAR_POSITION.TOP || activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM,
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
                "px-1.5": activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT,
                "py-[3px]":
                  activityBarPosition === ACTIVITYBAR_POSITION.TOP ||
                  activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM,
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
