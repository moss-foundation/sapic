import { RefObject, useEffect, useState } from "react";

import {
  attachInstruction,
  extractInstruction,
  type Instruction,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { GroupedEnvironments } from "../types";

interface UseDraggableGroupedEnvironmentsListProps {
  ref: RefObject<HTMLUListElement | null>;
  groupWithEnvironments: GroupedEnvironments;
}

export const useDraggableGroupedEnvironmentsList = ({
  ref,
  groupWithEnvironments,
}: UseDraggableGroupedEnvironmentsListProps) => {
  const [isChildDropBlocked, setIsChildDropBlocked] = useState<boolean | null>(null);
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = ref?.current;
    if (!element) return;

    return combine(
      dropTargetForElements({
        element,
        getData: ({ input }) => {
          const data = {
            type: "GroupedEnvironmentList",
            data: { groupWithEnvironments },
          };

          return attachInstruction(data, {
            input,
            element,
            operations: {
              "reorder-before": "not-available",
              "reorder-after": "not-available",
              combine: "available",
            },
          });
        },

        onDrag: ({ location }) => {
          if (location.current.dropTargets.length > 1 || location.current.dropTargets.length === 0) {
            setIsChildDropBlocked(null);
            setInstruction(null);
            return;
          }
          setInstruction(extractInstruction(location.current.dropTargets[0].data));
          setIsChildDropBlocked(false);
        },
        onDrop: () => {
          setIsChildDropBlocked(null);
          setInstruction(null);
        },
        onDropTargetChange: () => {
          setIsChildDropBlocked(null);
          setInstruction(null);
        },
      })
    );
  }, [ref, groupWithEnvironments]);

  return { isChildDropBlocked, instruction };
};
