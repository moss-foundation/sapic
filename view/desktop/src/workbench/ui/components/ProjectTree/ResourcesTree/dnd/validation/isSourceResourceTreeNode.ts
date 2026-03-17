import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { ProjectDragType } from "../../../constants";

export const isSourceResourceNode = (source: ElementDragPayload): boolean => {
  return source.data.type === ProjectDragType.NODE;
};
