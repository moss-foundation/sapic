import { RefObject, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { DragGlobalEnvironmentsListItem, DropGlobalEnvironmentsListItem } from "../types";
import { getSourceGlobalEnvironmentsListItem } from "../utils";

interface UseDraggableGlobalEnvironmentsListProps {
  ref: RefObject<HTMLLIElement | null>;
  environment: StreamEnvironmentsEvent;
}

export const useDraggableGlobalEnvironmentsList = ({ ref, environment }: UseDraggableGlobalEnvironmentsListProps) => {
  const [isDragging, setIsDragging] = useState(false);
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = ref?.current;
    if (!element) return;

    return combine(
      draggable({
        element,
        getInitialData: (): DragGlobalEnvironmentsListItem => ({
          type: "GlobalEnvironmentsListItem",
          data: { environment },
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
          const data: DropGlobalEnvironmentsListItem = {
            type: "GlobalEnvironmentsListItem",
            data: { environment },
          };

          const sourceData = getSourceGlobalEnvironmentsListItem(source);
          if (!sourceData || sourceData.data.environment.id === environment.id) {
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
          const sourceData = getSourceGlobalEnvironmentsListItem(source);
          if (!sourceData) return false;

          const sameEnvironment = sourceData.data.environment.id === environment.id;
          return !sameEnvironment;
        },
        onDragEnter({ self }) {
          const instruction = extractInstruction(self.data);

          if (instruction) {
            setInstruction(instruction);
          }
        },
        onDrag({ self }) {
          const instruction = extractInstruction(self.data);

          if (instruction) {
            setInstruction(instruction);
          }
        },
        onDragLeave() {
          setInstruction(null);
        },
        onDrop: () => {
          setInstruction(null);
        },
      })
    );
  }, [ref, environment]);

  return { isDragging, instruction };
};
