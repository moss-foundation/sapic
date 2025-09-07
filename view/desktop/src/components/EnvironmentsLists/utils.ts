import { extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { DragEnvironmentItem, DropEnvironmentItem, DropOperation, GlobalEnvironmentItem } from "./types";

//source
export const getSourceEnvironmentItem = (source: ElementDragPayload): DragEnvironmentItem | null => {
  if (source.data.type !== "GlobalEnvironmentItem" && source.data.type !== "GroupedEnvironmentItem") {
    return null;
  }

  return source.data as unknown as DragEnvironmentItem;
};

export const isSourceEnvironmentItem = (source: ElementDragPayload): boolean => {
  return source.data.type === "GlobalEnvironmentItem" || source.data.type === "GroupedEnvironmentItem";
};

export const isSourceGlobalEnvironmentItem = (source: ElementDragPayload): boolean => {
  return source.data.type === "GlobalEnvironmentItem";
};

export const isSourceGroupedEnvironmentItem = (source: ElementDragPayload): boolean => {
  return source.data.type === "GroupedEnvironmentItem";
};

export const getSourceGlobalEnvironmentItemData = (source: ElementDragPayload): GlobalEnvironmentItem | null => {
  if (source.data.type !== "GlobalEnvironmentItem") {
    return null;
  }

  return source.data as unknown as GlobalEnvironmentItem;
};

//location
export const getLocationEnvironmentItemData = (location: DragLocationHistory): DropEnvironmentItem | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (
    location.current.dropTargets[0].data.type !== "GlobalEnvironmentItem" &&
    location.current.dropTargets[0].data.type !== "GroupedEnvironmentItem"
  )
    return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    "data": {
      ...(location.current.dropTargets[0].data.data as DropEnvironmentItem["data"]),
    },
    "type": location.current.dropTargets[0].data.type,
    "instruction": instruction ?? undefined,
  };
};

//other

export const getDropOperation = (
  source: DragEnvironmentItem,
  location: DropEnvironmentItem,
  instruction: Instruction
): DropOperation | null => {
  if (!instruction) return null;

  if (instruction.operation === "combine") {
    if (location.type === "GroupedEnvironmentList") {
      return "CombineGrouped";
    }
  } else {
    if (source.type === "GlobalEnvironmentItem" && location.type === "GlobalEnvironmentItem") {
      return "ReorderGlobal";
    }

    if (source.type === "GroupedEnvironmentItem" && location.type === "GroupedEnvironmentItem") {
      return "ReorderGrouped";
    }

    if (source.type === "GlobalEnvironmentItem" && location.type === "GroupedEnvironmentItem") {
      return "MoveToGrouped";
    }

    if (source.type === "GroupedEnvironmentItem" && location.type === "GlobalEnvironmentItem") {
      return "MoveToGlobal";
    }
  }
  return null;
};
