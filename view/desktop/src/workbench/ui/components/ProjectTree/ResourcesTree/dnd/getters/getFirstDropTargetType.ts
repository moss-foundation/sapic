import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { ProjectDragType } from "../../../constants";

export const getFirstDropTargetType = (location: DragLocationHistory): ProjectDragType | null => {
  if (location.current.dropTargets.length === 0) return null;
  return location.current.dropTargets[0].data.type as ProjectDragType;
};
