import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { LocationResourcesListData } from "../../../dnd/types.dnd";
import { DraggedResourceNode } from "../../../types";
import { hasDirectDescendantWithSimilarName } from "../../../utils";

export const canCombineToResourcesList = (
  sourceData: DraggedResourceNode | null,
  locationData: LocationResourcesListData
): Availability => {
  if (!sourceData) {
    return "not-available";
  }

  if (hasDirectDescendantWithSimilarName(locationData.data.resourcesTree, sourceData.node)) {
    return "blocked";
  }

  return "available";
};
