import { Dispatch, SetStateAction, useEffect } from "react";

import { CreateNewCollectionFromTreeNodeEvent, TreeNodeProps, TreeProps } from "../types";
import { removeNodeFromTree, removeUniqueIdFromTree } from "../utils";

interface useCreateNewCollectionFromTreeNodeProps {
  treeId: TreeProps["id"];
  onNodeAdd: TreeProps["onNodeAdd"];
  onNodeRemove: TreeProps["onNodeRemove"];
  onRootAdd: TreeProps["onRootAdd"];
  onRootRemove: TreeProps["onRootRemove"];
  onTreeUpdate: TreeProps["onTreeUpdate"];

  setTree: Dispatch<SetStateAction<TreeNodeProps>>;
}

export const useCreateNewCollectionFromTreeNodeEvent = ({
  treeId,
  onNodeAdd,
  onNodeRemove,
  onRootAdd,
  onRootRemove,
  onTreeUpdate,
  setTree,
}: useCreateNewCollectionFromTreeNodeProps) => {
  useEffect(() => {
    const handleCreateNewCollectionFromTreeNode = (event: CustomEvent<CreateNewCollectionFromTreeNodeEvent>) => {
      const { source } = event.detail;
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
    };

    window.addEventListener("createNewCollectionFromTreeNode", handleCreateNewCollectionFromTreeNode as EventListener);

    return () => {
      window.removeEventListener(
        "createNewCollectionFromTreeNode",
        handleCreateNewCollectionFromTreeNode as EventListener
      );
    };
  }, [onNodeAdd, onNodeRemove, onRootAdd, onRootRemove, onTreeUpdate, treeId]);
};
