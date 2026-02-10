import { RefObject, useEffect, useState } from "react";

import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ENVIRONMENT_ITEM_DRAG_TYPE, ENVIRONMENT_LIST_DRAG_TYPE } from "../../constants";
import {
  getLocationEnvironmentItemData,
  getLocationProjectEnvironmentListData,
  getSourceEnvironmentItemData,
} from "../../EnvironmentItem/dnd/getters";
import { DropProjectEnvironmentList } from "../../EnvironmentItem/dnd/types.dnd";
import {
  isLocationEnvironmentItem,
  isLocationProjectEnvironmentList,
  isSourceEnvironmentItem,
} from "../../EnvironmentItem/dnd/validation";
import { canCombineToEnvironmentList } from "../../EnvironmentItem/dnd/validation/canCombineToEnvironmentList";

interface UseDropTargetProjectEnvironmentListProps {
  refList: RefObject<HTMLUListElement | null>;
  refListHeader: RefObject<HTMLLIElement | null>;
  projectId: string;
  projectEnvironments: EnvironmentSummary[];
}

export const useDropTargetProjectEnvironmentList = ({
  refList,
  refListHeader,
  projectId,
  projectEnvironments,
}: UseDropTargetProjectEnvironmentListProps) => {
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = refList.current;
    const elementHeader = refListHeader.current;
    if (!element || !elementHeader) return;

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
  }, [instruction, projectEnvironments, projectId, refList, refListHeader]);

  return {
    instruction,
  };
};
