import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { ProjectDragType } from "../../../constants";
import { DragResourceNodeData } from "../types.dnd";

export const getSourceProjectTreeNodeData = (source: ElementDragPayload): DragResourceNodeData | null => {
  if (source.data.type !== ProjectDragType.NODE) {
    return null;
  }

  return source.data.data as DragResourceNodeData;
};
