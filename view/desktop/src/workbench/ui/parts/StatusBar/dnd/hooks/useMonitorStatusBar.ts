import { useEffect } from "react";

import { useBatchPutStatusBarItemState } from "@/workbench/adapters/tanstackQuery";
import { swapListById } from "@/workbench/utils";
import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { useSyncStatusBarItems } from "../../hooks/useSyncStatusBarItems";
import { StatusBarItem } from "../../types";
import { isSourceStatusBarButton } from "../validation/isSourceStatusBarButton";

export const useMonitorStatusBar = () => {
  const { items, isLoading } = useSyncStatusBarItems();
  const { mutateAsync: batchPutStatusBarItemState } = useBatchPutStatusBarItemState();

  useEffect(() => {
    if (isLoading) return;

    return monitorForElements({
      canMonitor({ source }) {
        return isSourceStatusBarButton(source);
      },
      async onDrop({ location, source }) {
        const target = location.current.dropTargets[0];
        if (!target) return;

        const sourceData = source.data.data as StatusBarItem | undefined;
        const targetData = target.data.data as StatusBarItem | undefined;

        if (!sourceData || !targetData) return;

        const edge = extractClosestEdge(target.data);
        const itemsWithOrder = items.map(
          (item) => ({ ...item, order: item.order ?? null }) as StatusBarItem & { order: number | null }
        );
        const reorderedItems = swapListById(sourceData.id, targetData.id, itemsWithOrder, edge);

        if (!reorderedItems) return;

        const itemsToUpdate = reorderedItems.filter((reorderedItem) => {
          const item = items.find((i) => i.id === reorderedItem.id);
          if (!item) return false;
          return item.order !== reorderedItem.order;
        });

        if (itemsToUpdate.length === 0) return;

        const statusBarItemStates: Record<string, number> = Object.fromEntries(
          itemsToUpdate.map((item) => [String(item.id), item.order!])
        );

        await batchPutStatusBarItemState({ statusBarItemStates });
      },
    });
  }, [batchPutStatusBarItemState, items, isLoading]);
};
