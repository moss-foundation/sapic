import { RefObject, useContext, useEffect, useState } from "react";

import { attachClosestEdge, extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeContext } from "../Tree";
import { TreeCollectionRootNode } from "../types";

export const useDraggableRootNode = (
  draggableRef: RefObject<HTMLDivElement>,
  node: TreeCollectionRootNode,
  isRenamingNode: boolean
) => {
  const { id } = useContext(TreeContext);

  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [canDrop, setCanDrop] = useState<boolean>(false);

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
            collectionId: id,
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
              collectionId: id,
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
          return source.data.type === "TreeRootNode";
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
        onDrop() {
          setClosestEdge(null);
        },
      })
    );
  }, [node, isRenamingNode, draggableRef, closestEdge, id]);

  return {
    closestEdge,
    setClosestEdge,
    isDragging,
  };
};
