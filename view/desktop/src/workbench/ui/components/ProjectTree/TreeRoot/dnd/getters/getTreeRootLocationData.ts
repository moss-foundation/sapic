import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DragTreeRootData } from "../types.dnd";

export const getTreeRootLocationData = (location: DragLocationHistory) => {
  return location.current?.dropTargets[0].data as DragTreeRootData;
};
