import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DragTreeRootNodeData } from "../types.dnd";

export const getTreeRootNodeSourceData = (source: ElementDragPayload): DragTreeRootNodeData => {
  return source.data as DragTreeRootNodeData;
};
