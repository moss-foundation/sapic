import React from "react";

import { DragNode } from "@/components/CollectionTree/types";
import { getSourceTreeNodeData, isSourceTreeNode } from "@/components/CollectionTree/utils";
import { dropTargetForElements, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

export const useTabbedPaneDropTarget = (
  dockviewRef: React.RefObject<HTMLDivElement>,
  setPragmaticDropElement: React.Dispatch<React.SetStateAction<DragNode | null>>
) => {
  const [canDrop, setCanDrop] = React.useState(true);
  const [isDragging, setIsDragging] = React.useState(false);

  React.useEffect(() => {
    if (!dockviewRef.current) return;

    const evaluateDropTarget = ({ source }: { source: ElementDragPayload }) => {
      setIsDragging(true);

      if (isSourceTreeNode(source)) {
        setCanDrop(true);

        const sourceTarget = getSourceTreeNodeData(source);
        if (sourceTarget) {
          setPragmaticDropElement(sourceTarget);
        } else {
          setPragmaticDropElement(null);
          setCanDrop(false);
        }
      } else {
        setCanDrop(false);
      }
    };

    const clearDropTarget = () => {
      setIsDragging(false);
      // setPragmaticDropElement(null);
      setCanDrop(true);
    };

    return dropTargetForElements({
      element: dockviewRef.current,
      onDragEnter: evaluateDropTarget,
      onDragStart: evaluateDropTarget,
      onDragLeave: clearDropTarget,
      onDrop: clearDropTarget,
    });
  }, [dockviewRef, setPragmaticDropElement]);

  return { canDrop, isDragging };
};
