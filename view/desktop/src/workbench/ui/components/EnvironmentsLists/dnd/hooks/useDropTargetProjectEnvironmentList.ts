import { RefObject, useEffect, useState } from "react";

import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ENVIRONMENT_LIST_DRAG_TYPE } from "../../constants";
import { getSourceEnvironmentItemData } from "../getters";
import { DropProjectEnvironmentList } from "../types.dnd";
import { canCombineToEnvironmentList } from "../validation/canCombineToEnvironmentList";
import { isLocationEnvironmentItem } from "../validation/isLocationEnvironmentItem";
import { isLocationProjectEnvironmentList } from "../validation/isLocationProjectEnvironmentList";
import { isSourceEnvironmentItem } from "../validation/isSourceEnvironmentItem";

interface UseDropTargetProjectEnvironmentListProps {
  refList: RefObject<HTMLDivElement | null>;
  projectId: string;
  projectEnvironments: EnvironmentSummary[];
}

export const useDropTargetProjectEnvironmentList = ({
  refList,
  projectId,
  projectEnvironments,
}: UseDropTargetProjectEnvironmentListProps) => {
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
        const locationData: DropProjectEnvironmentList = {
          type: ENVIRONMENT_LIST_DRAG_TYPE.PROJECT,
          data: {
            projectId,
            projectEnvironments,
          },
        };

        return attachInstruction(locationData, {
          input,
          element,
          operations: {
            "reorder-before": "not-available",
            "reorder-after": "not-available",
            combine: canCombineToEnvironmentList({ environments: projectEnvironments, sourceData }),
          },
        });
      },
      onDrag({ self, location }) {
        const instruction = extractInstruction(self.data);

        if (isLocationProjectEnvironmentList(location)) {
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
  }, [instruction, projectEnvironments, projectId, refList]);

  return {
    instruction,
  };
};
