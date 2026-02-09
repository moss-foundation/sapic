import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

export const getSourceEnvironmentItemData = (source: ElementDragPayload): DragEnvironmentItem => {
  return source.data as unknown as DragEnvironmentItem;
};

export const getLocationEnvironmentItemData = (location: DragLocationHistory): DropEnvironmentItem | null => {
  return location.current.dropTargets[0].data as unknown as DropEnvironmentItem;
};
