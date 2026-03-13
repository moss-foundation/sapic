import { ResourceNode } from "../../ResourcesTree/types";
import { ProjectTreeRoot } from "../../types";

export const checkIfAllFoldersAreCollapsed = (tree: ProjectTreeRoot): boolean => {
  const checkIfAllNodesAreCollapsed = (node: ResourceNode): boolean => {
    if (!node || node.kind === "Item") return true;

    if (node.expanded) return false;

    if (!node.childNodes || node.childNodes.length === 0) return true;

    return node.childNodes.every(checkIfAllNodesAreCollapsed);
  };

  if (!tree.resourcesTree.childNodes || tree.resourcesTree.childNodes.length === 0) return true;

  return tree.resourcesTree.childNodes.every(checkIfAllNodesAreCollapsed);
};
