import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DragEnvironmentItem, DropEnvironmentItem, DropProjectEnvironmentList } from "./types.dnd";

export const getSourceEnvironmentItemData = (source: ElementDragPayload): DragEnvironmentItem => {
  return source.data as unknown as DragEnvironmentItem;
};

export const getLocationEnvironmentItemData = (location: DragLocationHistory): DropEnvironmentItem | null => {
  if (location.current.dropTargets.length === 0) return null;
  return location.current.dropTargets[0].data as unknown as DropEnvironmentItem;
};

export const getLocationProjectEnvironmentListData = (
  location: DragLocationHistory
): DropProjectEnvironmentList | null => {
  if (location.current.dropTargets.length === 0) return null;
  return location.current.dropTargets[0].data as unknown as DropProjectEnvironmentList;
};
