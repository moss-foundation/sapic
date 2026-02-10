import { RefObject, useEffect, useState } from "react";

import { useGetAllProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetAllProjectEnvironments";
import { useGetWorkspaceEnvironments } from "@/db/environmentsSummaries/hooks/useGetWorkspaceEnvironments";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../../constants";
import { getSourceEnvironmentItemData } from "../getters";
import { DragEnvironmentItem } from "../types.dnd";
import { canReorderEnvironmentList } from "../validation/canReorderEnvironmentList";
import { isSourceEnvironmentItem } from "../validation/isSourceEnvironmentItem";

interface UseDraggableEnvironmentItemProps {
  ref: RefObject<HTMLLIElement | null>;
  environment: EnvironmentSummary;
  type: ENVIRONMENT_ITEM_DRAG_TYPE;
  canDrag?: boolean;
}

export const useDraggableEnvironmentItem = ({
  ref,
  environment,
  type,
  canDrag = true,
}: UseDraggableEnvironmentItemProps) => {
  const [isDragging, setIsDragging] = useState(false);
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  const { workspaceEnvironments } = useGetWorkspaceEnvironments();
  const { projectEnvironments: allProjectEnvironments } = useGetAllProjectEnvironments();

  useEffect(() => {
    const element = ref?.current;
    if (!element) return;

    return combine(
      draggable({
        element,
        canDrag: () => canDrag,
        getInitialData: (): DragEnvironmentItem => ({
          type,
          data: environment,
        }),
        onDragStart() {
          setIsDragging(true);
        },
        onDrop() {
          setIsDragging(false);
        },
      }),
      dropTargetForElements({
        element,
        canDrop: ({ source }) => {
          return isSourceEnvironmentItem(source);
        },
        getData: ({ input, element, source }) => {
          const locationData: DragEnvironmentItem = {
            type,
            data: environment,
          };
          const sourceData = getSourceEnvironmentItemData(source);
          const canReorderResult = canReorderEnvironmentList(
            sourceData,
            locationData,
            workspaceEnvironments,
            allProjectEnvironments
          );

          return attachInstruction(locationData, {
            input,
            element,
            operations: {
              "reorder-before": canReorderResult,
              "reorder-after": canReorderResult,
              combine: "not-available",
            },
          });
        },
        onDragEnter({ self }) {
          setInstruction(extractInstruction(self.data));
        },
        onDrag({ self }) {
          setInstruction(extractInstruction(self.data));
        },
        onDragLeave() {
          setInstruction(null);
        },
        onDrop: () => {
          setInstruction(null);
        },
      })
    );
  }, [environment, canDrag, ref, type, workspaceEnvironments, allProjectEnvironments]);

  return {
    instruction,
    isDragging,
  };
};
