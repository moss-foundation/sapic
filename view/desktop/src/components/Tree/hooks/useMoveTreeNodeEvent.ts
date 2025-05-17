import { Dispatch, SetStateAction, useEffect } from "react";

import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { MoveNodeEventDetail, TreeNodeProps, TreeProps } from "../types";
import {
  addNodeChildrenWithInstruction,
  addNodeToFolder,
  findParentNodeByChildUniqueId,
  hasDescendant,
  removeNodeFromTree,
  removeUniqueIdFromTree,
} from "../utils";

interface useMoveTreeNodeProps {
  treeId: TreeProps["id"];
  onNodeAdd: TreeProps["onNodeAdd"];
  onNodeRemove: TreeProps["onNodeRemove"];
  onRootAdd: TreeProps["onRootAdd"];
  onRootRemove: TreeProps["onRootRemove"];
  onTreeUpdate: TreeProps["onTreeUpdate"];

  setTree: Dispatch<SetStateAction<TreeNodeProps>>;
}

const addNodeToTreeWithInstruction = (
  tree: TreeNodeProps,
  targetNode: TreeNodeProps,
  sourceNode: TreeNodeProps,
  instruction: Instruction | undefined
): TreeNodeProps => {
  const treeWithoutSource = removeNodeFromTree(tree, sourceNode.uniqueId);

  if (!instruction) {
    if (targetNode.isFolder) {
      return addNodeToFolder(treeWithoutSource, targetNode.uniqueId, sourceNode);
    }

    return tree;
  }

  if (instruction.operation === "combine" && targetNode.isFolder) {
    return addNodeToFolder(treeWithoutSource, targetNode.uniqueId, sourceNode);
  }

  const targetParentNode = findParentNodeByChildUniqueId(treeWithoutSource, targetNode.uniqueId);
  if (!targetParentNode) return treeWithoutSource;

  const indexOfTargetNode = targetParentNode.childNodes.findIndex((child) => child.uniqueId === targetNode.uniqueId);
  if (indexOfTargetNode === -1) return treeWithoutSource;

  if (instruction.operation === "reorder-before") {
    return addNodeChildrenWithInstruction(
      treeWithoutSource,
      targetParentNode.uniqueId,
      [
        ...targetParentNode.childNodes.slice(0, indexOfTargetNode),
        sourceNode,
        ...targetParentNode.childNodes.slice(indexOfTargetNode),
      ],
      instruction
    );
  }

  if (instruction.operation === "reorder-after") {
    return addNodeChildrenWithInstruction(
      treeWithoutSource,
      targetParentNode.uniqueId,
      [
        ...targetParentNode.childNodes.slice(0, indexOfTargetNode + 1),
        sourceNode,
        ...targetParentNode.childNodes.slice(indexOfTargetNode + 1),
      ],
      instruction
    );
  }

  return tree;
};

export const useMoveTreeNodeEvent = ({
  treeId,
  onNodeAdd,
  onNodeRemove,
  onRootAdd,
  onRootRemove,
  onTreeUpdate,
  setTree,
}: useMoveTreeNodeProps) => {
  useEffect(() => {
    const handleMoveTreeNode = (event: CustomEvent<MoveNodeEventDetail>) => {
      const { source, target, instruction } = event.detail;

      if (source.node.uniqueId === target.node.uniqueId) return;

      if (source.treeId === target.treeId && source.treeId === treeId) {
        if (hasDescendant(source.node, target.node)) {
          return;
        }

        setTree((prevTree) => {
          return addNodeToTreeWithInstruction(prevTree, target.node, source.node, instruction);
        });
      } else {
        if (target.treeId === treeId) {
          setTree((prevTree) => {
            const updatedTree = addNodeToTreeWithInstruction(prevTree, target.node, source.node, instruction);
            if (source.node.isRoot) {
              onRootAdd?.(source.node);
            } else {
              onNodeAdd?.(source.node);
            }
            onTreeUpdate?.(removeUniqueIdFromTree(updatedTree));
            return updatedTree;
          });
        }
        if (source.treeId === treeId) {
          setTree((prevTree) => {
            const removedTree = removeNodeFromTree(prevTree, source.node.uniqueId);
            if (source.node.isRoot) {
              onRootRemove?.(source.node);
            } else {
              onNodeRemove?.(source.node);
            }
            onTreeUpdate?.(removeUniqueIdFromTree(removedTree));
            return removedTree;
          });
        }
      }
    };

    window.addEventListener("moveTreeNode", handleMoveTreeNode as EventListener);
    return () => {
      window.removeEventListener("moveTreeNode", handleMoveTreeNode as EventListener);
    };
  }, [onNodeAdd, onNodeRemove, onRootAdd, onRootRemove, onTreeUpdate, treeId]);
};
