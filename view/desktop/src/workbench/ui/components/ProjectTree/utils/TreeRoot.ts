import { extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { ProjectDragType } from "../constants";
import { ProjectTreeRootNode, ResourceNode } from "../types";

export const isSourceTreeRootNode = (source: ElementDragPayload): boolean => {
  return source.data.type === ProjectDragType.ROOT_NODE;
};

export const checkIfAllFoldersAreExpanded = (tree: ProjectTreeRootNode): boolean => {
  const checkIfAllNodesAreExpanded = (node: ResourceNode): boolean => {
    if (!node || node.kind === "Item") return true;

    if (!node.expanded) return false;

    if (!node.childNodes || node.childNodes.length === 0) return true;

    return node.childNodes.every(checkIfAllNodesAreExpanded);
  };

  if (!tree.childNodes || tree.childNodes.length === 0) return true;

  return tree.childNodes.every(checkIfAllNodesAreExpanded);
};

export const checkIfAllFoldersAreCollapsed = (tree: ProjectTreeRootNode): boolean => {
  const checkIfAllNodesAreCollapsed = (node: ResourceNode): boolean => {
    if (!node || node.kind === "Item") return true;

    if (node.expanded) return false;

    if (!node.childNodes || node.childNodes.length === 0) return true;

    return node.childNodes.every(checkIfAllNodesAreCollapsed);
  };

  if (!tree.childNodes || tree.childNodes.length === 0) return true;

  return tree.childNodes.every(checkIfAllNodesAreCollapsed);
};

export const getTreeRootNodeTargetData = (location: DragLocationHistory) => {
  const instruction = extractInstruction(location.current?.dropTargets[0].data);

  return {
    type: "TreeRootNode",
    data: {
      ...location.current?.dropTargets[0].data,
      instruction,
    },
  } as {
    type: "TreeRootNode";
    data: {
      instruction: Instruction;
      projectId: string;
      node: ProjectTreeRootNode;
    };
  };
};

export const getChildrenNames = (node: ResourceNode | ProjectTreeRootNode) => {
  return node.childNodes.map((childNode) => childNode.name);
};
