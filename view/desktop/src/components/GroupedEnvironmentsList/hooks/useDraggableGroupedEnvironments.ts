import { RefObject, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { DragGroupedEnvironmentsListItem, DropGroupedEnvironmentsListItem, GroupedWithEnvironment } from "../types";
import { getSourceGroupedEnvironmentsListItem } from "../utils";

interface UseDraggableGroupedEnvironmentsListProps {
  ref: RefObject<HTMLLIElement | null>;
  groupWithEnvironments: GroupedWithEnvironment;
}

export const useDraggableGroupedEnvironmentsList = ({
  ref,
  groupWithEnvironments,
}: UseDraggableGroupedEnvironmentsListProps) => {
  const [isDragging, setIsDragging] = useState(false);
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = ref?.current;

    if (!element) return;

    return combine(
      draggable({
        element,
        getInitialData: (): DragGroupedEnvironmentsListItem => ({
          type: "GroupedEnvironmentsListItem",
          data: { groupWithEnvironments },
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
        getData({ input, source }) {
          const data: DropGroupedEnvironmentsListItem = {
            type: "GroupedEnvironmentsListItem",
            data: { groupWithEnvironments },
          };

          const sourceData = getSourceGroupedEnvironmentsListItem(source);
          if (
            !sourceData ||
            sourceData.data.groupWithEnvironments.collectionId === groupWithEnvironments.collectionId
          ) {
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
              "reorder-before": "available",
              "reorder-after": "available",
              combine: "not-available",
            },
          });
        },
        getIsSticky() {
          return true;
        },
        canDrop({ source }) {
          const sourceData = getSourceGroupedEnvironmentsListItem(source);
          if (!sourceData) return false;

          const sameEnvironment =
            sourceData.data.groupWithEnvironments.collectionId === groupWithEnvironments.collectionId;
          return !sameEnvironment;
        },
        onDragEnter({ self }) {
          const instruction = extractInstruction(self.data);
          setInstruction(instruction);
        },
        onDrag({ self }) {
          const instruction = extractInstruction(self.data);
          setInstruction(instruction);
        },
        onDragLeave() {
          setInstruction(null);
        },
        onDrop: () => {
          setInstruction(null);
        },
      })
    );
  }, [ref, groupWithEnvironments]);

  return { isDragging, instruction };
};
