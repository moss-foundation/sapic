/* eslint-disable */
import { Dispatch, SetStateAction, useContext, useEffect } from "react";

import { TreeContext } from "../Tree";
import { MoveNodeEventDetail, TreeNodeProps, TreeProps } from "../types";
import { addNodeToTreeWithInstruction, hasDescendant, removeNodeFromTree } from "../utils";

interface useMoveTreeNodeProps {
  onNodeAdd: TreeProps["onNodeAdd"];
  onNodeRemove: TreeProps["onNodeRemove"];
  onRootAdd: TreeProps["onRootAdd"];
  onRootRemove: TreeProps["onRootRemove"];
  onTreeUpdate: TreeProps["onTreeUpdate"];

  setTree: Dispatch<SetStateAction<TreeNodeProps>>;
}

export const useMoveTreeNodeEvent = ({
  onNodeAdd,
  onNodeRemove,
  onRootAdd,
  onRootRemove,
  onTreeUpdate,
  setTree,
}: useMoveTreeNodeProps) => {
  const { id } = useContext(TreeContext);
  useEffect(() => {
    const handleMoveTreeNode = (event: CustomEvent<MoveNodeEventDetail>) => {
      const { source, target, instruction } = event.detail;

      if (source.node.uniqueId === target.node.uniqueId) return;

      if (source.treeId === target.treeId && source.treeId === id) {
        if (hasDescendant(source.node, target.node)) {
          return;
        }

        setTree((prevTree) => {
          return addNodeToTreeWithInstruction(prevTree, target.node, source.node, instruction);
        });
      } else {
        if (target.treeId === id) {
          setTree((prevTree) => {
            const updatedTree = addNodeToTreeWithInstruction(prevTree, target.node, source.node, instruction);
            if (source.node.isRoot) {
              onRootAdd?.(source.node);
            } else {
              onNodeAdd?.(source.node);
            }
            onTreeUpdate?.(updatedTree);
            return updatedTree;
          });
        }
        if (source.treeId === id) {
          setTree((prevTree) => {
            const removedTree = removeNodeFromTree(prevTree, source.node.uniqueId);
            if (source.node.isRoot) {
              onRootRemove?.(source.node);
            } else {
              onNodeRemove?.(source.node);
            }
            onTreeUpdate?.(removedTree);
            return removedTree;
          });
        }
      }
    };

    window.addEventListener("moveTreeNode", handleMoveTreeNode as EventListener);
    return () => {
      window.removeEventListener("moveTreeNode", handleMoveTreeNode as EventListener);
    };
  }, [onNodeAdd, onNodeRemove, onRootAdd, onRootRemove, onTreeUpdate, setTree, treeId]);
};
