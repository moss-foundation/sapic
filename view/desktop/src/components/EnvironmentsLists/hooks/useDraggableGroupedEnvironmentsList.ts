import { RefObject, useEffect, useState } from "react";

import {
  attachInstruction,
  extractInstruction,
  type Instruction,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ENVIRONMENT_LIST_DRAG_TYPE } from "../constants";
import { GroupedEnvironments } from "../types";
import {
  getSourceEnvironmentItem,
  hasSimilarEnv,
  isSourceEnvironmentItem,
  isSourceGroupedEnvironmentList,
} from "../utils";

interface UseDraggableGroupedEnvironmentsListProps {
  ref: RefObject<HTMLUListElement | null>;
  groupWithEnvironments: GroupedEnvironments;
}

export const useDraggableGroupedEnvironmentsList = ({
  ref,
  groupWithEnvironments,
}: UseDraggableGroupedEnvironmentsListProps) => {
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = ref?.current;
    if (!element) return;

    return combine(
      dropTargetForElements({
        element,
        canDrop: ({ source }) => {
          return isSourceEnvironmentItem(source) || isSourceGroupedEnvironmentList(source);
        },
        getData: ({ input, source }) => {
          const data = {
            type: ENVIRONMENT_LIST_DRAG_TYPE.GROUPED,
            data: { groupWithEnvironments },
          };

          const sourceData = getSourceEnvironmentItem(source);
          if (!sourceData) {
            return attachInstruction(data, {
              input,
              element,
              operations: {
                "reorder-before": "not-available",
                "reorder-after": "not-available",
                combine: "not-available",
              },
            });
          }

          return attachInstruction(data, {
            input,
            element,
            operations: {
              "reorder-before": "not-available",
              "reorder-after": "not-available",
              combine: hasSimilarEnv(groupWithEnvironments, sourceData.data.environment) ? "blocked" : "available",
            },
          });
        },
        onDrag: ({ location }) => {
          if (location.current.dropTargets.length > 1 || location.current.dropTargets.length === 0) {
            setInstruction(null);
            return;
          }

          setInstruction(extractInstruction(location.current.dropTargets[0].data));
        },
        onDrop: () => {
          setInstruction(null);
        },
        onDropTargetChange: () => {
          setInstruction(null);
        },
      })
    );
  }, [ref, groupWithEnvironments]);

  return { instruction };
};
