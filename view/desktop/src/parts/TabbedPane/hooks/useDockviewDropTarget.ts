import React from "react";

import { DropNodeElement } from "@/components/Tree/types";
import { getActualDropSourceTarget } from "@/components/Tree/utils";
import { dropTargetForElements, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

export const useTabbedPaneDropTarget = (
  dockviewRef: React.RefObject<HTMLDivElement>,
  setPragmaticDropElement: React.Dispatch<React.SetStateAction<DropNodeElement | null>>
) => {
  const [canDrop, setCanDrop] = React.useState(false);
  const [isDragging, setIsDragging] = React.useState(false);

  const evaluateDropTarget = ({ source }: { source: ElementDragPayload }) => {
    setIsDragging(true);

    const sourceTarget = getActualDropSourceTarget(source);

    if (sourceTarget?.node?.type === "TreeNode" || sourceTarget?.node?.uniqueId) {
      setCanDrop(true);
    } else {
      setCanDrop(false);
      return;
    }

    if (source) setPragmaticDropElement(sourceTarget);
    else {
      setPragmaticDropElement(null);
      setCanDrop(false);
    }
  };

  const clearDropTarget = () => {
    setIsDragging(false);
    setPragmaticDropElement(null);
    setCanDrop(false);
  };

  React.useEffect(() => {
    if (!dockviewRef.current) return;

    const dropTarget = dropTargetForElements({
      element: dockviewRef.current,
      onDragEnter: evaluateDropTarget,
      onDragStart: evaluateDropTarget,
      onDragLeave: clearDropTarget,
      onDrop: clearDropTarget,
    });

    return () => dropTarget();
  }, [dockviewRef, setPragmaticDropElement]);

  return { canDrop, isDragging };
};
