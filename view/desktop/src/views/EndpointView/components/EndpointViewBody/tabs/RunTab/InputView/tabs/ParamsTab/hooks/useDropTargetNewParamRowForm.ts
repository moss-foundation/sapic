import { RefObject, useEffect, useState } from "react";

import { attachClosestEdge, extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { DROP_TARGET_NEW_PARAM_ROW_FORM_TYPE, ParamDragType } from "../constants";
import { isSourceParamRow } from "../utils/dragAndDrop";

interface UseDropTargetNewParamRowFormProps {
  newParamRowFormRef: RefObject<HTMLDivElement | null>;
  resourceId: string;
  paramType: ParamDragType;
}

export const useDropTargetNewParamRowForm = ({
  newParamRowFormRef,
  resourceId,
  paramType,
}: UseDropTargetNewParamRowFormProps) => {
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  useEffect(() => {
    const element = newParamRowFormRef.current;
    if (!element) return;

    return dropTargetForElements({
      element: element,
      canDrop: ({ source }) => {
        return isSourceParamRow(source);
      },
      getData({ input }) {
        return attachClosestEdge(
          {
            type: DROP_TARGET_NEW_PARAM_ROW_FORM_TYPE,
            data: {
              resourceId,
              paramType,
            },
          },
          { element, input, allowedEdges: ["top"] }
        );
      },
      onDrop: () => {
        setClosestEdge(null);
      },
      onDragLeave: () => {
        setClosestEdge(null);
      },
      onDragEnter({ self }) {
        const closestEdge = extractClosestEdge(self.data);
        setClosestEdge(closestEdge);
      },
      onDrag({ self }) {
        const closestEdge = extractClosestEdge(self.data);
        setClosestEdge(closestEdge);
      },
    });
  }, [resourceId, newParamRowFormRef, paramType]);

  return { closestEdge };
};
