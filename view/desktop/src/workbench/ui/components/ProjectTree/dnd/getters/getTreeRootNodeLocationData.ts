import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DragTreeRootNodeData } from "../types.dnd";

export const getTreeRootNodeLocationData = (location: DragLocationHistory) => {
  return location.current?.dropTargets[0].data as DragTreeRootNodeData;
};
