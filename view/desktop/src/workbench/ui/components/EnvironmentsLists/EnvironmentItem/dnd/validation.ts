import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../constants";

export const isSourceEnvironmentItem = (source: ElementDragPayload): boolean => {
  return (
    source.data.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT || source.data.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE
  );
};

//validation is not actually finished
export const canCombine = (source: ElementDragPayload): Availability => {
  if (isSourceEnvironmentItem(source)) return "not-available";
  return "not-available";
};
