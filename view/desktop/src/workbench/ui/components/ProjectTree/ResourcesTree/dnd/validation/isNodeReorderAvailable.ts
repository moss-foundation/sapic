import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DragNode, ResourceNode } from "../../../types";
import { hasDescendant, hasDirectDescendantWithSimilarName } from "../../../utils/TreeProjectNode";

export const isNodeReorderAvailable = (sourceTarget: DragNode | null, locationNode: ResourceNode): Availability => {
  if (!sourceTarget || !locationNode) {
    return "not-available";
  }

  if (sourceTarget.node.id === locationNode.id) {
    return "not-available";
  }

  if (locationNode.kind === "Dir" && locationNode.expanded) {
    return "not-available";
  }

  if (sourceTarget.node.class !== locationNode.class) {
    return "blocked";
  }

  if (hasDescendant(sourceTarget.node, locationNode)) {
    return "blocked";
  }

  if (hasDirectDescendantWithSimilarName(locationNode, sourceTarget.node)) {
    return "blocked";
  }

  return "available";
};
