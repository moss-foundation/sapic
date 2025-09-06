import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

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
