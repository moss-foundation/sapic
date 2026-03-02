import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DropResourcesList } from "../../../dnd/types.dnd";
import { DragNode } from "../../../types";

export const canCombineToResourcesList = (
  sourceData: DragNode | null,
  locationData: DropResourcesList
): Availability => {
  if (!sourceData) {
    return "not-available";
  }

  if (locationData.data.tree.childNodes.some((child) => child.id === sourceData.node.id)) {
    return "blocked";
  }

  return "available";
};
