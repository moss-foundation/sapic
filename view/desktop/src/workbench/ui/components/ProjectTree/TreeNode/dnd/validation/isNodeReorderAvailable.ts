import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DragNode, DropNode } from "../../../types";
import { hasDescendant, hasDirectDescendantWithSimilarName } from "../../../utils/TreeProjectNode";

export const isNodeReorderAvailable = (sourceTarget: DragNode, dropTarget: DropNode): Availability => {
  if (sourceTarget.node.id === dropTarget.node.id) {
    return "not-available";
  }

  if (dropTarget.node.kind === "Dir" && dropTarget.node.expanded) {
    return "not-available";
  }

  if (sourceTarget.node.class !== dropTarget.node.class) {
    return "blocked";
  }

  if (hasDescendant(sourceTarget.node, dropTarget.node)) {
    return "blocked";
  }

  if (hasDirectDescendantWithSimilarName(dropTarget.node, sourceTarget.node)) {
    return "blocked";
  }

  return "available";
};
