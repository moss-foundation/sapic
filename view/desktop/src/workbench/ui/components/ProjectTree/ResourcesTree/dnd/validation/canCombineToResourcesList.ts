import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DraggedResourceNode } from "../../../types";
import { LocationResourcesListData } from "../types.dnd";

export const canCombineToResourcesList = (
  sourceData: DraggedResourceNode | null,
  locationData: LocationResourcesListData
): Availability => {
  if (!sourceData) {
    return "not-available";
  }

  const { rootResourcesNodes } = locationData.data;

  const hasSameId = rootResourcesNodes.some((node) => node.id === sourceData.node.id);
  const hasSimilarName = rootResourcesNodes.some(
    (node) => node.name.toLowerCase() === sourceData.node.name.toLowerCase()
  );

  if (hasSameId || hasSimilarName) {
    return "blocked";
  }

  return "available";
};
