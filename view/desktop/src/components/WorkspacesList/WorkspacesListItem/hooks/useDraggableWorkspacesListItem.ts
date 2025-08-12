import { useEffect, useState } from "react";

import { attachClosestEdge, Edge, extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { DragWorkspacesListItem } from "../types";
import { getSourceWorkspacesListItem } from "../utils";

interface UseDraggableWorkspacesListItemProps {
  ref: React.RefObject<HTMLDivElement>;
  environment: StreamEnvironmentsEvent;
}

export const useDraggableWorkspacesListItem = ({ ref, environment }: UseDraggableWorkspacesListItemProps) => {
  const [isDragging, setIsDragging] = useState(false);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  useEffect(() => {
    const element = ref.current;

    if (!element) return;

    return combine(
      draggable({
        element,
        getInitialData: (): DragWorkspacesListItem => ({
          type: "WorkspacesListItem",
          data: { environment },
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
          const data: DragWorkspacesListItem = {
            type: "WorkspacesListItem",
            data: { environment },
          };

          return attachClosestEdge(data, {
            input,
            element,
            allowedEdges: ["top", "bottom"],
          });
        },
        canDrop({ source }) {
          const sourceData = getSourceWorkspacesListItem(source);

          if (!sourceData) return false;

          const sameEnvironment = sourceData.data.environment.id === environment.id;

          return !sameEnvironment;
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
  }, [ref, environment]);

  return { isDragging, closestEdge };
};
