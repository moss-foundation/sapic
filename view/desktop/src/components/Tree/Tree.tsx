import { createContext, useEffect, useId, useState } from "react";

import { TreeRootNode } from "./TreeRootNode.tsx";
import {
  CreateNewCollectionFromTreeNodeEvent,
  MoveNodeEventDetail,
  TreeContextProps,
  TreeNodeProps,
  TreeProps,
} from "./types.ts";
import {
  addNodeToFolder,
  checkIfAllFoldersAreCollapsed,
  checkIfAllFoldersAreExpanded,
  hasDescendant,
  prepareCollectionForTree,
  removeNodeFromTree,
  removeUniqueIdFromTree,
  sortNode,
  updateTreeNode,
} from "./utils.ts";

export const TreeContext = createContext<TreeContextProps>({
  treeId: "",
  paddingLeft: 0,
  paddingRight: 0,
  nodeOffset: 0,
  allFoldersAreExpanded: false,
  allFoldersAreCollapsed: true,
  searchInput: undefined,
});

export const Tree = ({
  id,
  tree: initialTree,
  paddingLeft = 16,
  paddingRight = 8,
  nodeOffset = 12,
  searchInput,

  onTreeUpdate,

  onRootAdd,
  onRootRemove,
  onRootRename,
  onRootUpdate,
  onRootClick,
  onRootDoubleClick,

  onNodeAdd,
  onNodeRemove,
  onNodeRename,
  onNodeUpdate,
  onNodeClick,
  onNodeDoubleClick,
}: TreeProps) => {
  const reactUniqueId = useId();
  const treeId = id || reactUniqueId;
  const [tree, setTree] = useState<TreeNodeProps>(prepareCollectionForTree(initialTree));

  const handleNodeUpdate = (updatedNode: TreeNodeProps) => {
    setTree((prev) => updateTreeNode(prev, updatedNode));
    onTreeUpdate?.(removeUniqueIdFromTree(updatedNode));

    if (updatedNode.isRoot) onRootUpdate?.(updatedNode);
    else onNodeUpdate?.(updatedNode);
  };

  useEffect(() => {
    const handleMoveTreeNode = (event: CustomEvent<MoveNodeEventDetail>) => {
      const { source, target } = event.detail;
      if (source.treeId === target.treeId && source.treeId === treeId) {
        if (hasDescendant(source.node, target.node) || source.node.uniqueId === target.node.uniqueId) {
          return;
        }
        setTree((prevTree) => {
          const treeWithoutSource = removeNodeFromTree(prevTree, source.node.uniqueId);
          const updatedTree = addNodeToFolder(treeWithoutSource, target.node.uniqueId, source.node);
          return sortNode(updatedTree);
        });
      } else {
        if (target.treeId === treeId) {
          setTree((prevTree) => {
            const updatedTree = addNodeToFolder(prevTree, target.node.uniqueId, source.node);
            if (source.node.isRoot) {
              onRootAdd?.(source.node);
            } else {
              onNodeAdd?.(source.node);
            }
            return sortNode(updatedTree);
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
            return removedTree;
          });
        }
      }
    };

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
          return removedTree;
        });
      }
    };

    window.addEventListener("moveTreeNode", handleMoveTreeNode as EventListener);
    window.addEventListener("createNewCollectionFromTreeNode", handleCreateNewCollectionFromTreeNode as EventListener);

    return () => {
      window.removeEventListener("moveTreeNode", handleMoveTreeNode as EventListener);
      window.removeEventListener(
        "createNewCollectionFromTreeNode",
        handleCreateNewCollectionFromTreeNode as EventListener
      );
    };
  }, [onNodeAdd, onNodeRemove, onRootAdd, onRootRemove, treeId]);

  return (
    <TreeContext.Provider
      value={{
        treeId,
        paddingLeft,
        paddingRight,
        nodeOffset,
        allFoldersAreExpanded: checkIfAllFoldersAreExpanded(tree.childNodes),
        allFoldersAreCollapsed: checkIfAllFoldersAreCollapsed(tree.childNodes),
        searchInput,

        onRootAddCallback: onRootAdd,
        onRootRemoveCallback: onRootRemove,
        onRootRenameCallback: onRootRename,
        onRootUpdateCallback: onRootUpdate,
        onRootClickCallback: onRootClick,
        onRootDoubleClickCallback: onRootDoubleClick,

        onNodeAddCallback: onNodeAdd,
        onNodeRemoveCallback: onNodeRemove,
        onNodeRenameCallback: onNodeRename,
        onNodeUpdateCallback: onNodeUpdate,
        onNodeClickCallback: onNodeClick,
        onNodeDoubleClickCallback: onNodeDoubleClick,
      }}
    >
      <div className="select-none">
        <TreeRootNode onNodeUpdate={handleNodeUpdate} node={tree} />
      </div>
    </TreeContext.Provider>
  );
};

export default Tree;
