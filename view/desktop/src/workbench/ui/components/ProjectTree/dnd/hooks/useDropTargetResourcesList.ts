import { RefObject, useEffect, useState } from "react";

import {
  attachInstruction,
  Availability,
  extractInstruction,
  Instruction,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectDragType } from "../../constants";
import { DragNode, ProjectTreeRootNode } from "../../types";
import { getSourceProjectTreeNodeData, isSourceProjectTreeNode } from "../../utils/DragAndDrop";
import { DropResourcesList } from "../types.dnd";

interface UseDropTargetResourcesListProps {
  ref: RefObject<HTMLDivElement | null>;
  tree: ProjectTreeRootNode;
}

export const useDropTargetResourcesList = ({ ref, tree }: UseDropTargetResourcesListProps) => {
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = ref.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      canDrop: ({ source }) => {
        return isSourceProjectTreeNode(source);
      },
      getData: ({ input, element, source }) => {
        const sourceData = getSourceProjectTreeNodeData(source);
        const locationData: DropResourcesList = {
          type: ProjectDragType.RESOURCES_LIST,
          data: { tree },
        };

        return attachInstruction(locationData, {
          input,
          element,
          operations: {
            "reorder-before": "not-available",
            "reorder-after": "not-available",
            combine: canCombineToResourcesList(sourceData, locationData),
          },
        });
      },

      onDrag: ({ self }) => {
        setInstruction(extractInstruction(self.data));
      },
      onDragLeave: () => {
        setInstruction(null);
      },
      onDrop: () => {
        setInstruction(null);
      },
    });
  }, [ref, tree]);

  return {
    instruction,
  };
};

//TODO: move to utils
const canCombineToResourcesList = (sourceData: DragNode | null, locationData: DropResourcesList): Availability => {
  if (!sourceData) {
    return "not-available";
  }

  if (locationData.data.tree.childNodes.some((child) => child.id === sourceData.node.id)) {
    return "blocked";
  }

  return "available";
};
