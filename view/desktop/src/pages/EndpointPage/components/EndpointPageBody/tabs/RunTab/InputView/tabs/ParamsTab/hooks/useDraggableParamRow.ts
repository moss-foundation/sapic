import { useEffect, useRef, useState } from "react";

import { attachClosestEdge, extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { QueryParamInfo } from "@repo/moss-project";

import { DRAGGABLE_PARAM_ROW_TYPE, DROP_TARGET_PARAM_ROW_TYPE } from "../constants";
import { DraggableParamRowData, ParamDragType } from "../types";
import { getDraggableParamRowSourceData } from "../utils/dragAndDrop";

interface UseDraggableParamRowProps {
  param: QueryParamInfo;
  resourceId: string;
  paramType: ParamDragType;
}

export const useDraggableParamRow = ({ param, resourceId, paramType }: UseDraggableParamRowProps) => {
  const dragHandleRef = useRef<HTMLDivElement>(null);
  const paramRowRef = useRef<HTMLDivElement>(null);

  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);
  const [isDragging, setIsDragging] = useState(false);

  useEffect(() => {
    const element = paramRowRef.current;
    const dragHandle = dragHandleRef.current;
    if (!element || !dragHandle) return;

    return combine(
      draggable({
        element,
        dragHandle,
        getInitialData: (): DraggableParamRowData => ({
          type: DRAGGABLE_PARAM_ROW_TYPE,
          data: { param, paramType, resourceId },
        }),
        onDragStart: () => setIsDragging(true),
        onDrop: () => setIsDragging(false),
      }),
      dropTargetForElements({
        element,
        canDrop({ source }) {
          if (source.data.type !== DRAGGABLE_PARAM_ROW_TYPE) {
            return false;
          }

          const sourceData = getDraggableParamRowSourceData(source);
          if (!sourceData) {
            return false;
          }

          if (sourceData.data.param.id === param.id) {
            return false;
          }

          return true;
        },
        getIsSticky: () => true,
        getData({ input }) {
          return attachClosestEdge(
            { type: DROP_TARGET_PARAM_ROW_TYPE, data: { param, paramType, resourceId } },
            { element, input, allowedEdges: ["top", "bottom"] }
          );
        },
        onDrop: () => setClosestEdge(null),
        onDragEnter({ self }) {
          const edge = extractClosestEdge(self.data);
          setClosestEdge(edge);
        },
        onDrag({ self }) {
          const edge = extractClosestEdge(self.data);
          setClosestEdge((current) => (current === edge ? current : edge));
        },
        onDragLeave: () => setClosestEdge(null),
      })
    );
  }, [param, resourceId, paramType]);

  return { isDragging, dragHandleRef, paramRowRef, closestEdge };
};
