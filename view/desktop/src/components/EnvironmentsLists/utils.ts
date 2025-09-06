import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { DragEnvironmentItem, DropEnvironmentItem, DropOperation } from "./types";

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
    ...(location.current.dropTargets[0].data.data as DragEnvironmentItem),
    "type": location.current.dropTargets[0].data.type,
    "instruction": instruction ?? undefined,
  };
};

//other

export const getDropOperation = (location: ElementDragPayload): DropOperation | null => {
  if (location.data.type !== "GlobalEnvironmentItem" && location.data.type !== "GroupedEnvironmentItem") {
    return null;
  }

  return location.data as unknown as DropOperation;
};
