import { RefObject, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectDragType } from "../../../constants";
import { LocationResourcesListData } from "../../../dnd/types.dnd";
import { ResourcesTree } from "../../../types";
import { getSourceProjectTreeNodeData, isSourceProjectTreeNode } from "../../../utils/DragAndDrop";
import { canCombineToResourcesList } from "../validation/canCombineToResourcesList";

interface UseDropTargetResourcesListProps {
  ref: RefObject<HTMLDivElement | null>;
  resourcesTree: ResourcesTree;
}

export const useDropTargetResourcesList = ({ ref, resourcesTree }: UseDropTargetResourcesListProps) => {
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = ref.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      canDrop: ({ source }) => isSourceProjectTreeNode(source),
      getData: ({ input, element, source }) => {
        const sourceData = getSourceProjectTreeNodeData(source);
        const locationData: LocationResourcesListData = {
          type: ProjectDragType.RESOURCES_LIST,
          data: { resourcesTree },
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
  }, [ref, resourcesTree]);

  return {
    instruction,
  };
};
