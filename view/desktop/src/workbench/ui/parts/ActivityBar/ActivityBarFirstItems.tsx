import { useEffect } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { cn } from "@/utils";
import { useGetLayout } from "@/workbench/adapters";
import { useBatchPutActivityBarItemState } from "@/workbench/adapters/tanstackQuery/activityBarItemState/useBatchPutActivityBarItemState";
import { ACTIVITYBAR_POSITION } from "@/workbench/domains/layout";
import { swapListById } from "@/workbench/utils";
import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ActivityBarButton } from "./ActivityBarButton";
import { ActivityBarButtonIndicator } from "./ActivityBarButtonIndicator";
import { ActivityBarButtonProps } from "./types";
import { useActivityBarFirstItems } from "./useActivityBarFirstItems";

export const ActivityBarFirstItems = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { data: layout } = useGetLayout();
  const { items, isLoadingActivityBarItemStates } = useActivityBarFirstItems();
  const { mutateAsync: batchPutActivityBarItemState } = useBatchPutActivityBarItemState();

  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return source.data.type === "ActivityBarButton";
      },
      onDrop({ location, source }) {
        const target = location.current.dropTargets[0];
        if (!target) return;

        const sourceData = source.data as { data: ActivityBarButtonProps };
        const targetData = target.data as { data: ActivityBarButtonProps };
        const edge = extractClosestEdge(targetData);

        if (!sourceData || !targetData || !sourceData.data || !targetData.data) return;

        const updatedItems = swapListById(sourceData.data.id, targetData.data.id, items, edge);

        if (!updatedItems) return;

        batchPutActivityBarItemState({
          activityBarItemStates: updatedItems.map((item) => ({ id: item.id, order: item.order })),
          workspaceId: currentWorkspaceId,
        });
      },
    });
  }, [batchPutActivityBarItemState, currentWorkspaceId, items]);

  if (isLoadingActivityBarItemStates) return null;

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
