import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { DragEnvironmentItem, DragGlobalEnvironmentsListItem, DropGlobalEnvironmentsListItem } from "../types";

//source
export const isSourceGlobalEnvironmentsListItem = (source: ElementDragPayload) => {
  return source.data.type === "GlobalEnvironmentsListItem";
};

export const getSourceGlobalEnvironmentsListItem = (
  source: ElementDragPayload
): DragGlobalEnvironmentsListItem | null => {
  if (!isSourceGlobalEnvironmentsListItem(source)) {
    return null;
  }

  return source.data as unknown as DragGlobalEnvironmentsListItem;
};

export const getSourceGlobalEnvironmentsListData = (source: ElementDragPayload): StreamEnvironmentsEvent | null => {
  if (!isSourceGlobalEnvironmentsListItem(source)) {
    return null;
  }

  return source.data.environment as DragGlobalEnvironmentsListItem["data"]["environment"];
};

export const isSourceEnvironmentItem = (source: ElementDragPayload) => {
  return source.data.type === "EnvironmentItem";
};

export const getSourceEnvironmentItem = (source: ElementDragPayload): DragEnvironmentItem | null => {
  if (!isSourceEnvironmentItem(source)) {
    return null;
  }

  return source.data as unknown as DragEnvironmentItem;
};

//location
export const isLocationGlobalEnvironmentsListItem = (location: DragLocationHistory) => {
  return location.current.dropTargets.some((target) => target.data.type === "GlobalEnvironmentsListItem");
};

export const getLocationGlobalEnvironmentsListItem = (
  location: DragLocationHistory
): DropGlobalEnvironmentsListItem | null => {
  if (!isLocationGlobalEnvironmentsListItem(location)) {
    return null;
  }

  const closestEdge = extractClosestEdge(location.current.dropTargets[0]?.data);

  return {
    ...location.current.dropTargets[0]?.data,
    edge: closestEdge,
  } as unknown as DropGlobalEnvironmentsListItem;
};

export const getLocationGlobalEnvironmentsListItemData = (
  location: DragLocationHistory
): StreamEnvironmentsEvent | null => {
  if (!isLocationGlobalEnvironmentsListItem(location)) {
    return null;
  }

  return location.current.dropTargets[0]?.data.environment as unknown as StreamEnvironmentsEvent;
};
