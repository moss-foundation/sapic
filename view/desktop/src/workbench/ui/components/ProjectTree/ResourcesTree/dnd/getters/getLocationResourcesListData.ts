import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { ProjectDragType } from "../../../constants";
import { LocationResourcesListData } from "../types.dnd";

export const getLocationResourcesListData = (location: DragLocationHistory): LocationResourcesListData | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== ProjectDragType.RESOURCES_LIST) return null;

  return location.current.dropTargets[0].data as LocationResourcesListData;
};
