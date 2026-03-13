import React from "react";

import { getSourceProjectTreeNodeData } from "@/workbench/ui/components/ProjectTree/ResourcesTree/dnd/getters/getSourceProjectTreeNodeData.ts";
import { DragResourceNodeData } from "@/workbench/ui/components/ProjectTree/ResourcesTree/dnd/types.dnd";
import { isSourceResourceNode } from "@/workbench/ui/components/ProjectTree/ResourcesTree/dnd/validation/isSourceResourceTreeNode.ts";
import { dropTargetForElements, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

export const useTabbedPaneDropTarget = (
  dockviewRef: React.RefObject<HTMLDivElement | null>,
  setPragmaticDropElement: React.Dispatch<React.SetStateAction<DragResourceNodeData | null>>
) => {
  const [canDrop, setCanDrop] = React.useState(true);
  const [isDragging, setIsDragging] = React.useState(false);

  React.useEffect(() => {
    if (!dockviewRef.current) return;

    const evaluateDropTarget = ({ source }: { source: ElementDragPayload }) => {
      setIsDragging(true);

      if (isSourceResourceNode(source)) {
        setCanDrop(true);

        const sourceTarget = getSourceProjectTreeNodeData(source);
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
