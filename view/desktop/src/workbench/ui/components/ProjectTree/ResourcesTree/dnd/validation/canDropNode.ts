import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DraggedResourceNode, DropNode } from "../../../types";
import { hasDirectDescendantWithSimilarName } from "../../../utils";

/**
 * @deprecated we use instructions now to determine if a node can be dropped
 */
export const canDropNode = (sourceTarget: DraggedResourceNode, dropTarget: DropNode, operation: Operation) => {
  if (sourceTarget.node.class !== dropTarget.node.class) {
    return false;
  }

  if (sourceTarget.node.id === dropTarget.node.id) {
    return false;
  }

  if (dropTarget.node.kind === "Item") {
    if (hasDirectDescendantWithSimilarName(dropTarget.parentNode, sourceTarget.node)) {
      return false;
    }
  }

  if (dropTarget.node.kind === "Dir") {
    if (operation === "combine") {
      if (hasDirectDescendantWithSimilarName(dropTarget.node, sourceTarget.node)) {
        return false;
      }
    } else {
      if (hasDirectDescendantWithSimilarName(dropTarget.parentNode, sourceTarget.node)) {
        return false;
      }
    }
  }

  return true;
};
