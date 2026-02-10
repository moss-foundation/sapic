import { RefObject, useEffect, useState } from "react";

import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ENVIRONMENT_LIST_DRAG_TYPE } from "../../constants";
import { getSourceEnvironmentItemData } from "../getters";
import { DropWorkspaceEnvironmentList } from "../types.dnd";
import { canCombineToEnvironmentList } from "../validation/canCombineToEnvironmentList";
import { isLocationEnvironmentItem } from "../validation/isLocationEnvironmentItem";
import { isLocationWorkspaceEnvironmentList } from "../validation/isLocationWorkspaceEnvironmentList";
import { isSourceEnvironmentItem } from "../validation/isSourceEnvironmentItem";

interface UseDropTargetWorkspaceEnvironmentListProps {
  refList: RefObject<HTMLUListElement | null>;
  workspaceEnvironments: EnvironmentSummary[];
}

export const useDropTargetWorkspaceEnvironmentList = ({
  refList,
  workspaceEnvironments,
}: UseDropTargetWorkspaceEnvironmentListProps) => {
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = refList.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      canDrop({ source }) {
        return isSourceEnvironmentItem(source);
      },
      getData({ input, element, source }) {
        const sourceData = getSourceEnvironmentItemData(source);
        const locationData: DropWorkspaceEnvironmentList = {
          type: ENVIRONMENT_LIST_DRAG_TYPE.WORKSPACE,
          data: { workspaceEnvironments },
        };

        return attachInstruction(locationData, {
          input,
          element,
          operations: {
            "reorder-before": "not-available",
            "reorder-after": "not-available",
            combine: canCombineToEnvironmentList({ environments: workspaceEnvironments, sourceData }),
          },
        });
      },
      onDrag({ self, location }) {
        const instruction = extractInstruction(self.data);

        if (isLocationWorkspaceEnvironmentList(location)) {
          setInstruction(instruction);
        }

        if (isLocationEnvironmentItem(location)) {
          setInstruction(instruction?.blocked ? instruction : null);
        }
      },
      onDragLeave() {
        setInstruction(null);
      },
      onDrop() {
        setInstruction(null);
      },
    });
  }, [instruction, refList, workspaceEnvironments]);

  return {
    instruction,
  };
};
