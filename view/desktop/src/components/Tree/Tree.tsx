import { createContext, useCallback, useEffect, useId, useState } from "react";

import TreeNode from "./TreeNode.tsx";
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
  horizontalPadding: 16,
  nodeOffset: 16,
  allFoldersAreExpanded: false,
  allFoldersAreCollapsed: true,
  searchInput: undefined,
});

export const Tree = ({
  id,
  tree: initialTree,
  horizontalPadding = 16,
  nodeOffset = 16,
  onTreeUpdate,
  searchInput,
}: TreeProps) => {
  const reactUniqueId = useId();
  const treeId = id || reactUniqueId;
  const [tree, setTree] = useState<TreeNodeProps>(prepareCollectionForTree(initialTree));

  const handleNodeUpdate = useCallback((updatedNode: TreeNodeProps) => {
    setTree((prev) => updateTreeNode(prev, updatedNode));
  }, []);

  useEffect(() => {
    onTreeUpdate?.(removeUniqueIdFromTree(tree));
  }, [onTreeUpdate, tree]);

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
            return sortNode(updatedTree);
          });
        }
        if (source.treeId === treeId) {
          setTree((prevTree) => removeNodeFromTree(prevTree, source.node.uniqueId));
        }
      }
    };

    const handleCreateNewCollectionFromTreeNode = (event: CustomEvent<CreateNewCollectionFromTreeNodeEvent>) => {
      const { source } = event.detail;
      if (source.treeId === treeId) {
        setTree((prevTree) => removeNodeFromTree(prevTree, source.node.uniqueId));
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
  }, [treeId]);

  return (
    <TreeContext.Provider
      value={{
        treeId,
        horizontalPadding,
        nodeOffset,
        allFoldersAreExpanded: checkIfAllFoldersAreExpanded(tree.childNodes),
        allFoldersAreCollapsed: checkIfAllFoldersAreCollapsed(tree.childNodes),
        searchInput,
      }}
    >
      <TreeNode parentNode={tree} onNodeUpdate={handleNodeUpdate} key={`root-${treeId}`} node={tree} depth={0} />
    </TreeContext.Provider>
  );
};

export default Tree;
