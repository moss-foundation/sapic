import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DragTreeRootData } from "../types.dnd";

export const getTreeRootSourceData = (source: ElementDragPayload): DragTreeRootData => {
  return source.data as DragTreeRootData;
};
