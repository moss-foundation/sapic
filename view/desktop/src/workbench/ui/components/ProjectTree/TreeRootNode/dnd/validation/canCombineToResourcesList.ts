import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DropResourcesList } from "../../../dnd/types.dnd";
import { DragNode } from "../../../types";
import { hasDirectDescendantWithSimilarName } from "../../../utils";

export const canCombineToResourcesList = (
  sourceData: DragNode | null,
  locationData: DropResourcesList
): Availability => {
  if (!sourceData) {
    return "not-available";
  }

  if (hasDirectDescendantWithSimilarName(locationData.data.tree, sourceData.node)) {
    return "blocked";
  }

  return "available";
};
