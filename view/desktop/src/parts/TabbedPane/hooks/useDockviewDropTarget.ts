import React from "react";

import { DropNodeElement } from "@/components/Tree/types";
import { getActualDropSourceTarget } from "@/components/Tree/utils";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

export const useTabbedPaneDropTarget = (
  dockviewRef: React.RefObject<HTMLDivElement>,
  setPragmaticDropElement: React.Dispatch<React.SetStateAction<DropNodeElement | null>>
) => {
  React.useEffect(() => {
    if (!dockviewRef.current) return;

    const dropTarget = dropTargetForElements({
      element: dockviewRef.current,
      onDragEnter: ({ source }) => {
        const sourceTarget = getActualDropSourceTarget(source);
        if (source) setPragmaticDropElement(sourceTarget);
      },
      canDrop: ({ source }) => source?.data?.type === "TreeNode",
      onDragLeave: () => setPragmaticDropElement(null),
    });

    return () => dropTarget();
  }, [dockviewRef, setPragmaticDropElement]);
};
