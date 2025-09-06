import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { DragEnvironmentItem } from "./EnvironmentItem/types";

//source
export const getSourceEnvironmentItemData = (source: ElementDragPayload): DragEnvironmentItem | null => {
  if (source.data.type !== "GlobalEnvItem" && source.data.type !== "GroupedEnvItem") {
    return null;
  }

  return source.data.data as DragEnvironmentItem;
};

export const isSourceEnvironmentItem = (source: ElementDragPayload): boolean => {
  return source.data.type === "GlobalEnvItem" || source.data.type === "GroupedEnvItem";
};

export const isSourceGlobalEnvironmentItem = (source: ElementDragPayload): boolean => {
  return source.data.type === "GlobalEnvItem";
};

export const isSourceGroupedEnvironmentItem = (source: ElementDragPayload): boolean => {
  return source.data.type === "GroupedEnvItem";
};
