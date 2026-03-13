import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { ProjectDragType } from "../constants";
import { ResourceNode } from "../ResourcesTree/types";
import { ProjectTreeRoot } from "../types";

export const isSourceTreeRoot = (source: ElementDragPayload): boolean => {
  return source.data.type === ProjectDragType.TREE_ROOT;
};

export const checkIfAllFoldersAreExpanded = (tree: ProjectTreeRoot): boolean => {
  const checkIfAllNodesAreExpanded = (node: ResourceNode): boolean => {
    if (!node || node.kind === "Item") return true;

    if (!node.expanded) return false;

    if (!node.childNodes || node.childNodes.length === 0) return true;

    return node.childNodes.every(checkIfAllNodesAreExpanded);
  };

  if (!tree.resourcesTree.childNodes || tree.resourcesTree.childNodes.length === 0) return true;

  return tree.resourcesTree.childNodes.every(checkIfAllNodesAreExpanded);
};

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

export const getChildrenNames = (node: ResourceNode) => {
  return node.childNodes.map((childNode) => childNode.name);
};
