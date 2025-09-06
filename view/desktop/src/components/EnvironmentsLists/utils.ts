import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { DragEnvironmentItem } from "./EnvironmentItem/types";

//source
export const getSourceEnvironmentItem = (source: ElementDragPayload): DragEnvironmentItem | null => {
  if (source.data.type !== "GlobalEnvironmentItem" && source.data.type !== "GroupedEnvironmentItem") {
    return null;
  }

  return source.data as unknown as DragEnvironmentItem;
};

export const isSourceEnvironmentItem = (source: ElementDragPayload): boolean => {
  return source.data.type === "GlobalEnvironmentItem" || source.data.type === "GroupedEnvironmentItem";
};

export const isSourceGlobalEnvironmentItem = (source: ElementDragPayload): boolean => {
  return source.data.type === "GlobalEnvironmentItem";
};

export const isSourceGroupedEnvironmentItem = (source: ElementDragPayload): boolean => {
  return source.data.type === "GroupedEnvironmentItem";
};
