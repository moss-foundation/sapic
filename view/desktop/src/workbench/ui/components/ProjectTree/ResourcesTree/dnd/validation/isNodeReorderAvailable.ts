import { Availability, Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { hasDescendant } from "../../../utils/TreeProjectNode";
import { DragResourceNode, DragResourceNodeData } from "../types.dnd";
import { hasPeersWithSimilarNameOrId } from "./hasPeersWithSimilarNameOrId";

export const isNodeReorderAvailable = (
  sourceData: DragResourceNodeData | null,
  locationData: DragResourceNode,
  operation: Omit<Operation, "combine">
): Availability => {
  if (!sourceData || !locationData) {
    return "not-available";
  }

  if (sourceData.node.id === locationData.data.node.id) {
    return "not-available";
  }

  if (locationData.data.node.kind === "Dir" && operation === "reorder-after") {
    return "not-available";
  }

  if (sourceData.node.class !== locationData.data.node.class) {
    return "blocked";
  }

  if (hasDescendant(sourceData.node, locationData.data.node)) {
    return "blocked";
  }

  if (hasPeersWithSimilarNameOrId({ sourceData, locationData })) {
    return "blocked";
  }

  return "available";
};
