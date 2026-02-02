import { RefObject, useEffect, useState } from "react";

import { useGetLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION } from "@/workbench/domains/layout";
import {
  attachClosestEdge,
  extractClosestEdge,
  type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { ACTIVITY_BAR_BUTTON_DND_TYPE } from "../constants";
import { ActivityBarButtonDragData } from "../types";

interface UseDraggableActivityBarButtonProps {
  id: string;
  order: number;
  isDraggable: boolean;
  ref: RefObject<HTMLButtonElement | null>;
}

export const useDraggableActivityBarButton = ({ id, order, isDraggable, ref }: UseDraggableActivityBarButtonProps) => {
  const { data: layout } = useGetLayout();

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;

  useEffect(() => {
    const element = ref.current;

    if (!element || !isDraggable) return;

    return combine(
      draggable({
        element: element,
        getInitialData: (): ActivityBarButtonDragData => ({
          type: ACTIVITY_BAR_BUTTON_DND_TYPE,
          data: { id, order },
        }),
        onDrop: () => {
          setPreview(null);
        },
        onGenerateDragPreview({ nativeSetDragImage }) {
          setCustomNativeDragPreview({
            nativeSetDragImage,
            render({ container }) {
              setPreview((prev) => (prev === container ? prev : container));
            },
          });
        },
      }),
      dropTargetForElements({
        element,
        getData({ input }) {
          return attachClosestEdge(
            { type: ACTIVITY_BAR_BUTTON_DND_TYPE, data: { id, order } },
            {
              element,
              input,
              allowedEdges:
                activityBarPosition === ACTIVITYBAR_POSITION.TOP || activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM
                  ? ["right", "left"]
                  : ["top", "bottom"],
            }
          );
        },
        canDrop({ source }) {
          return source.data.type === ACTIVITY_BAR_BUTTON_DND_TYPE;
        },
        getIsSticky() {
          return true;
        },
        onDragEnter({ self }) {
          const closestEdge = extractClosestEdge(self.data);
          setClosestEdge(closestEdge);
        },
        onDrag({ self }) {
          const closestEdge = extractClosestEdge(self.data);

          setClosestEdge((current) => {
            if (current === closestEdge) return current;

            return closestEdge;
          });
        },
        onDragLeave() {
          setClosestEdge(null);
        },
        onDrop: () => {
          setClosestEdge(null);
        },
      })
    );
  }, [activityBarPosition, id, order, isDraggable, ref]);

  return {
    preview,
    closestEdge,
  };
};
