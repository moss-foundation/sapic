import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { DragWorkspacesListItem, DropWorkspacesListItem } from "../types";

//source
export const isSourceWorkspacesListItem = (source: ElementDragPayload) => {
  return source.data.type === "WorkspacesListItem";
};

export const getSourceWorkspacesListItem = (source: ElementDragPayload): DragWorkspacesListItem | null => {
  if (!isSourceWorkspacesListItem(source)) {
    return null;
  }

  return source.data as unknown as DragWorkspacesListItem;
};

export const getSourceWorkspacesListItemData = (source: ElementDragPayload): StreamEnvironmentsEvent | null => {
  if (!isSourceWorkspacesListItem(source)) {
    return null;
  }

  return source.data.environment as DragWorkspacesListItem["data"]["environment"];
};

//location
export const isLocationWorkspacesListItem = (location: DragLocationHistory) => {
  return location.current.dropTargets.some((target) => target.data.type === "WorkspacesListItem");
};

export const getLocationWorkspacesListItem = (location: DragLocationHistory): DropWorkspacesListItem | null => {
  if (!isLocationWorkspacesListItem(location)) {
    return null;
  }

  const closestEdge = extractClosestEdge(location.current.dropTargets[0]?.data);

  return {
    ...location.current.dropTargets[0]?.data,
    edge: closestEdge,
  } as unknown as DropWorkspacesListItem;
};

export const getLocationWorkspacesListItemData = (location: DragLocationHistory): StreamEnvironmentsEvent | null => {
  if (!isLocationWorkspacesListItem(location)) {
    return null;
  }

  return location.current.dropTargets[0]?.data.environment as unknown as StreamEnvironmentsEvent;
};
