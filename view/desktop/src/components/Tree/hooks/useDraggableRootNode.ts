import { RefObject, useEffect, useState } from "react";

import { attachClosestEdge, extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { NodeProps } from "../types";

export const useDraggableRootNode = (
  draggableRef: RefObject<HTMLDivElement>,
  node: NodeProps,
  treeId: string | number,
  isRenamingNode: boolean
) => {
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);
  const [isDragging, setIsDragging] = useState<boolean>(false);

  useEffect(() => {
    const element = draggableRef.current;
    if (!element || isRenamingNode) return;

    return combine(
      draggable({
        element,
        getInitialData: () => ({
          type: "TreeRootNode",
          data: {
            node,
            treeId,
          },
        }),
        onDragStart() {
          setIsDragging(true);
        },
        onDrop() {
          setIsDragging(false);
        },
      }),
      dropTargetForElements({
        element,
        getData({ input }) {
          return attachClosestEdge(
            {
              node,
              treeId,
              closestEdge: closestEdge as Edge,
            },
            {
              element,
              input,
              allowedEdges: ["top", "bottom"],
            }
          );
        },
        canDrop({ source }) {
          return source.data.type === "TreeNodeRoot";
        },
        getIsSticky() {
          return true;
        },
        onDragEnter({ self }) {
          const closestEdge = extractClosestEdge(self.data);
          setClosestEdge(closestEdge);
        },
        onDrag({ self }) {
          console.log(1);
          const closestEdge = extractClosestEdge(self.data);

          setClosestEdge((current) => {
            if (current === closestEdge) return current;

            return closestEdge;
          });
        },
        onDragLeave() {
          setClosestEdge(null);
        },
        onDrop() {
          setClosestEdge(null);
        },
      })
    );
  }, [treeId, node, isRenamingNode, draggableRef, closestEdge]);

  return {
    closestEdge,
    setClosestEdge,
    isDragging,
  };
};
