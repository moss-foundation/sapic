import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DragGroupedEnvironmentsListItem, GroupedWithEnvironment } from "./types";

//source
export const isSourceGroupedEnvironmentsListItem = (source: ElementDragPayload) => {
  return source.data.type === "GroupedEnvironmentsListItem";
};

export const getSourceGroupedEnvironmentsListItem = (
  source: ElementDragPayload
): DragGroupedEnvironmentsListItem | null => {
  if (!isSourceGroupedEnvironmentsListItem(source)) {
    return null;
  }

  return source.data as unknown as DragGroupedEnvironmentsListItem;
};

export const getSourceGroupedEnvironmentsListData = (source: ElementDragPayload): GroupedWithEnvironment | null => {
  if (!isSourceGroupedEnvironmentsListItem(source)) {
    return null;
  }

  return source.data.groupWithEnvironments as DragGroupedEnvironmentsListItem["data"]["groupWithEnvironments"];
};
