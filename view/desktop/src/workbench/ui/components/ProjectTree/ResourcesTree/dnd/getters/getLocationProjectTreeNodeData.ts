import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { ProjectDragType } from "../../../constants";
import { DragResourceNodeData } from "../types.dnd";

export const getLocationProjectTreeNodeData = (location: DragLocationHistory): DragResourceNodeData | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== ProjectDragType.NODE) return null;

  return location.current.dropTargets[0].data.data as DragResourceNodeData;
};
