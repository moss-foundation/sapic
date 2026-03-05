import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectDragType } from "../../../constants";
import { ProjectTreeContext } from "../../../ProjectTreeContext";
import { ResourceNode } from "../../../types";
import { getSourceProjectTreeNodeData, isSourceProjectTreeNode } from "../../../utils/DragAndDrop";
import { LocationResourcesListData } from "../types.dnd";
import { canCombineToResourcesList } from "../validation/canCombineToResourcesList";

interface UseDropTargetResourcesListProps {
  ref: RefObject<HTMLDivElement | null>;
  rootResourcesNodes: ResourceNode[];
}

export const useDropTargetResourcesList = ({ ref, rootResourcesNodes }: UseDropTargetResourcesListProps) => {
  const { id } = useContext(ProjectTreeContext);

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
          data: { projectId: id, rootResourcesNodes },
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
  }, [ref, rootResourcesNodes, id]);

  return {
    instruction,
  };
};
