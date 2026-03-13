import { ResourceNode } from "../../ProjectTree/ResourcesTree/types";
import { collapseAllNodes } from "./collapseAllNodes";

export const closeAllNodesInTree = (tree: ResourceNode) => {
  const collapsedTree = { ...tree };
  return collapseAllNodes(collapsedTree);
};
