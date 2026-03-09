import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DraggedResourceNode } from "../../../types";
import { hasDescendant } from "../../../utils";
import { DragResourceNode } from "../types.dnd";
import { hasDirectSimilarDescendant } from "./hasDirectSimilarDescendant";

export const isNodeCombineAvailable = (
  sourceData: DraggedResourceNode | null,
  locationData: DragResourceNode
): Availability => {
  if (!sourceData || !locationData) {
    return "not-available";
  }

  if (locationData.data.node.kind !== "Dir") {
    return "not-available";
  }

  if (sourceData.node.id === locationData.data.node.id) {
    return "not-available";
  }

  if (sourceData.node.class !== locationData.data.node.class) {
    return "blocked";
  }

  if (hasDescendant(sourceData.node, locationData.data.node)) {
    return "not-available";
  }

  if (hasDirectSimilarDescendant({ locationData, sourceData })) {
    return "blocked";
  }

  return "available";
};
