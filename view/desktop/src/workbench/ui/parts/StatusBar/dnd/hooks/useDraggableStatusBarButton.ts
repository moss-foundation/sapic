import { RefObject, useEffect, useState } from "react";

import { attachClosestEdge, extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/types";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { STATUS_BAR_BUTTON_DND_TYPE } from "../../constants";
import { StatusBarItem } from "../../types";
import { StatusBarButtonDragData } from "../types.dnd";
import { isSourceStatusBarButton } from "../validation/isSourceStatusBarButton";

interface UseDraggableStatusBarButtonProps {
  ref: RefObject<HTMLButtonElement | null>;
  statusBarItem: StatusBarItem;
}

export const useDraggableStatusBarButton = ({ ref, statusBarItem }: UseDraggableStatusBarButtonProps) => {
  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  useEffect(() => {
    const element = ref.current;
    if (!element) return;

    return combine(
      draggable({
        element: element,
        getInitialData: (): StatusBarButtonDragData => ({
          type: STATUS_BAR_BUTTON_DND_TYPE,
          data: statusBarItem,
        }),
        onDrop: () => {
          setPreview(null);
        },
      }),
      dropTargetForElements({
        element,
        canDrop({ source }) {
          return isSourceStatusBarButton(source);
        },
        getData({ input }) {
          return attachClosestEdge(
            { type: STATUS_BAR_BUTTON_DND_TYPE, data: statusBarItem },
            {
              element,
              input,
              allowedEdges: ["right", "left"],
            }
          );
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
          setClosestEdge(closestEdge);
        },
        onDragLeave() {
          setClosestEdge(null);
        },
        onDrop: () => {
          setClosestEdge(null);
        },
      })
    );
  }, [statusBarItem, ref]);

  return {
    preview,
    closestEdge,
  };
};
