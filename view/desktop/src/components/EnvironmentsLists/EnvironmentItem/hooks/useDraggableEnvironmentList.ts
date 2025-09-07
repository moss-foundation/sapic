import { RefObject, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { DragEnvironmentItem, DropEnvironmentItem, EnvironmentListType } from "../../types";
import { getSourceEnvironmentItem } from "../../utils";

interface UseDraggableEnvironmentItemProps {
  ref: RefObject<HTMLLIElement | null>;
  environment: StreamEnvironmentsEvent;
  type: EnvironmentListType;
}

export const useDraggableEnvironmentItem = ({ ref, environment, type }: UseDraggableEnvironmentItemProps) => {
  const [isDragging, setIsDragging] = useState(false);
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = ref?.current;
    if (!element) return;

    return combine(
      draggable({
        element,
        getInitialData: (): DragEnvironmentItem => ({
          type,
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
          const data: DropEnvironmentItem = {
            type,
            data: { environment },
          };

          const sourceData = getSourceEnvironmentItem(source);
          if (!sourceData || sourceData.data.environment.id === environment.id) {
            return attachInstruction(data, {
              input,
              element,
              operations: {
                "reorder-before": "not-available",
                "reorder-after": "not-available",
                combine: "not-available",
              },
            }) as DropEnvironmentItem;
          }

          return attachInstruction(data, {
            input,
            element,
            operations: {
              "reorder-before": "available",
              "reorder-after": "available",
              combine: "not-available",
            },
          }) as DropEnvironmentItem;
        },
        canDrop({ source }) {
          const sourceData = getSourceEnvironmentItem(source);
          if (!sourceData) return false;

          const sameEnvironment = sourceData.data.environment.id === environment.id;
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
  }, [ref, environment, type]);

  return { isDragging, instruction };
};
