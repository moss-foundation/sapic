import { RefObject, useEffect, useState } from "react";

import {
  attachInstruction,
  extractInstruction,
  type Instruction,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { StreamEnvironmentsEvent } from "@repo/ipc";

import { ENVIRONMENT_LIST_DRAG_TYPE } from "../constants";
import { GroupedEnvironmentList } from "../types";
import {
  getSourceEnvironmentItem,
  getSourceGroupedEnvironmentListData,
  hasSimilarEnv,
  isSourceEnvironmentItem,
  isSourceGroupedEnvironmentList,
} from "../utils";

interface UseDraggableGroupedEnvironmentsListProps {
  ref: RefObject<HTMLUListElement | null>;
  environments: StreamEnvironmentsEvent[];
}

export const useDraggableGroupedEnvironmentsList = ({
  ref,
  environments,
}: UseDraggableGroupedEnvironmentsListProps) => {
  const [instruction, setInstruction] = useState<Instruction | null>(null);
  const [isDragging, setIsDragging] = useState<boolean>(false);

  useEffect(() => {
    const element = ref?.current;
    if (!element) return;

    return combine(
      draggable({
        element,
        getInitialData: (): GroupedEnvironmentList => ({
          type: ENVIRONMENT_LIST_DRAG_TYPE.GROUPED,
          data: { environments },
        }),
        onDragStart: () => setIsDragging(true),
        onDrop: () => setIsDragging(false),
      }),
      dropTargetForElements({
        element,
        canDrop: ({ source }) => {
          return isSourceEnvironmentItem(source) || isSourceGroupedEnvironmentList(source);
        },
        getData: ({ input, source }) => {
          const data = {
            type: ENVIRONMENT_LIST_DRAG_TYPE.GROUPED,
            data: { environments },
          };

          if (isSourceGroupedEnvironmentList(source)) {
            const sourceData = getSourceGroupedEnvironmentListData(source);
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
                "reorder-before":
                  sourceData.data.environments.projectId === environments.projectId ? "not-available" : "available",
                "reorder-after":
                  sourceData.data.environments.projectId === environments.projectId ? "not-available" : "available",
                combine: "not-available",
              },
            });
          }

          if (isSourceEnvironmentItem(source)) {
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
                combine: hasSimilarEnv(environments, sourceData.data.environment) ? "blocked" : "available",
              },
            });
          }

          return data;
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

  return { instruction, isDragging };
};
