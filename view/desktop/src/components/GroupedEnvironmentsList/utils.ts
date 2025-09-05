import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DragGroupedEnvironmentsListItem, DropGroupedEnvironmentsListItem, GroupedWithEnvironment } from "./types";

//source
export const isSourceGroupedEnvironmentsListItem = (source: ElementDragPayload) => {
  return source.data.type === "GroupedEnvironmentsListItem";
};

export const getSourceGroupedEnvironmentsListItem = (
  source: ElementDragPayload
): DragGroupedEnvironmentsListItem | null => {
  if (!isSourceGroupedEnvironmentsListItem(source)) {
    return null;
  }

  return source.data as unknown as DragGroupedEnvironmentsListItem;
};

export const getSourceGroupedEnvironmentsListData = (source: ElementDragPayload): GroupedWithEnvironment | null => {
  if (!isSourceGroupedEnvironmentsListItem(source)) {
    return null;
  }

  return source.data.groupWithEnvironments as DragGroupedEnvironmentsListItem["data"]["groupWithEnvironments"];
};

//location
export const getLocationGroupedEnvironmentsListItem = (
  location: DragLocationHistory
): DropGroupedEnvironmentsListItem | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== "GroupedEnvironmentsListItem") return null;

  return location.current.dropTargets[0].data as unknown as DropGroupedEnvironmentsListItem;
};
