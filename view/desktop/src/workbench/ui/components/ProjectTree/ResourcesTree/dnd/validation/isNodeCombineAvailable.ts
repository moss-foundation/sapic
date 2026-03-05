import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DragNode, ResourceNode } from "../../../types";
import { hasDescendant, hasDirectDescendantWithSimilarName, hasDirectSimilarDescendant } from "../../../utils";

export const isNodeCombineAvailable = (sourceTarget: DragNode | null, locationNode: ResourceNode): Availability => {
  if (!sourceTarget || !locationNode) {
    return "not-available";
  }

  if (locationNode.kind !== "Dir") {
    return "not-available";
  }

  if (sourceTarget.node.id === locationNode.id) {
    return "blocked";
  }

  if (sourceTarget.node.class !== locationNode.class) {
    return "blocked";
  }

  if (hasDescendant(sourceTarget.node, locationNode)) {
    return "blocked";
  }

  if (hasDirectSimilarDescendant(locationNode, sourceTarget.node)) {
    return "blocked";
  }

  return "available";
};

export const evaluateIsChildDropBlocked = (parentNode: ResourceNode, dropNode: ResourceNode): boolean => {
  if (parentNode.class !== dropNode.class) {
    return true;
  }

  if (hasDirectDescendantWithSimilarName(parentNode, dropNode)) {
    return true;
  }

  return false;
};
