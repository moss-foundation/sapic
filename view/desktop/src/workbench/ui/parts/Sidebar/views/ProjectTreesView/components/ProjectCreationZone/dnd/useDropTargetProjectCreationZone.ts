import { RefObject, useEffect, useState } from "react";

import { getSourceProjectTreeNodeData, isSourceProjectTreeNode } from "@/workbench/ui/components/ProjectTree/utils";
import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { PROJECT_CREATION_ZONE_TYPE } from "../constants";
import { DropProjectCreationZoneData } from "./types.dnd";
import { canCombineToProjectCreationZone } from "./validation/canCombineToProjectCreationZone";

interface UseDropTargetProjectCreationZoneProps {
  ref: RefObject<HTMLDivElement | null>;
}

export const useDropTargetProjectCreationZone = ({ ref }: UseDropTargetProjectCreationZoneProps) => {
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = ref.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      canDrop({ source }) {
        return isSourceProjectTreeNode(source);
      },
      getData: ({ input, source }) => {
        const sourceData = getSourceProjectTreeNodeData(source);
        const locationData: DropProjectCreationZoneData = { type: PROJECT_CREATION_ZONE_TYPE };

        return attachInstruction(locationData, {
          input,
          element,
          operations: {
            "reorder-before": "not-available",
            "reorder-after": "not-available",
            combine: canCombineToProjectCreationZone(sourceData),
          },
        });
      },
      onDrag({ self }) {
        const instruction = extractInstruction(self.data);
        setInstruction(instruction);
      },
      onDragLeave() {
        setInstruction(null);
      },
      onDrop() {
        setInstruction(null);
      },
    });
  }, [ref]);

  return {
    instruction,
  };
};
