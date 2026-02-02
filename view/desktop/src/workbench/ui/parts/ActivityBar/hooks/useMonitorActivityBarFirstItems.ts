import { useCurrentWorkspace } from "@/hooks";
import { useBatchPutActivityBarItemState } from "@/workbench/adapters/tanstackQuery/activityBarItemState/useBatchPutActivityBarItemState";
import { swapListById } from "@/workbench/utils";
import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { useEffect } from "react";
import { ACTIVITY_BAR_BUTTON_DND_TYPE } from "../constants";
import { ActivityBarButtonDragData } from "../types";
import { useSyncedActivityBarFirstItems } from "./useSyncedActivityBarFirstItems";

export const useMonitorActivityBarFirstItems = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { items } = useSyncedActivityBarFirstItems();
  const { mutateAsync: batchPutActivityBarItemState } = useBatchPutActivityBarItemState();

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return source.data.type === ACTIVITY_BAR_BUTTON_DND_TYPE;
      },
      async onDrop({ location, source }) {
        const target = location.current.dropTargets[0];
        if (!target) return;

        const sourceData = source.data.data as ActivityBarButtonDragData["data"];
        const targetData = target.data.data as ActivityBarButtonDragData["data"];

        if (!sourceData || !targetData) return;

        const edge = extractClosestEdge(target.data);
        const reorderedItems = swapListById(sourceData.id, targetData.id, items, edge);

        if (!reorderedItems) return;

        const itemsToUpdate = reorderedItems.filter((reorderedItem) => {
          const item = items.find((item) => item.id === reorderedItem.id);
          if (!item) return false;

          return item.order !== reorderedItem.order;
        });

        if (itemsToUpdate.length === 0) return;

        await batchPutActivityBarItemState({
          activityBarItemStates: itemsToUpdate.map((item) => ({ id: item.id, order: item.order })),
          workspaceId: currentWorkspaceId,
        });
      },
    });
  }, [batchPutActivityBarItemState, currentWorkspaceId, items]);
};
