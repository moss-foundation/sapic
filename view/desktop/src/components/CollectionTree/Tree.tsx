import { createContext, useEffect, useId, useState } from "react";

import { useCreateNewCollectionFromTreeNodeEvent } from "./hooks/useCreateNewCollectionFromTreeNodeEvent.ts";
import { useMoveTreeNodeEvent } from "./hooks/useMoveTreeNodeEvent.ts";
import { TreeRootNode } from "./TreeRootNode/TreeRootNode.tsx";
import { TreeContextProps, TreeNodeProps, TreeProps } from "./types.ts";
import {
  checkIfAllFoldersAreCollapsed,
  checkIfAllFoldersAreExpanded,
  prepareCollectionForTree,
  removeUniqueIdFromTree,
  updateTreeNode,
} from "./utils.ts";

export const TreeContext = createContext<TreeContextProps>({
  treeId: "",
  paddingLeft: 0,
  paddingRight: 0,
  rootOffset: 0,
  nodeOffset: 0,
  allFoldersAreExpanded: false,
  allFoldersAreCollapsed: true,
  searchInput: undefined,
  sortBy: "none",
});

export const CollectionTree = ({
  id,
  tree: initialTree,
  paddingLeft = 8,
  paddingRight = 8,
  rootOffset = 8,
  nodeOffset = 16,
  searchInput,
  sortBy = "none",

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
  const reactId = useId();
  const treeId = id || reactId;

  const [tree, setTree] = useState<TreeNodeProps>(prepareCollectionForTree(initialTree, sortBy));

  const handleNodeUpdate = (updatedNode: TreeNodeProps) => {
    setTree((prev) => {
      const newTree = updateTreeNode(prev, updatedNode);
      onTreeUpdate?.(removeUniqueIdFromTree(newTree));
      return newTree;
    });

    if (updatedNode.isRoot) onRootUpdate?.(updatedNode);
    else onNodeUpdate?.(updatedNode);
  };

  useEffect(() => {
    setTree(prepareCollectionForTree(initialTree, sortBy));
  }, [initialTree, sortBy]);

  useCreateNewCollectionFromTreeNodeEvent({
    treeId,
    onNodeAdd,
    onNodeRemove,
    onRootAdd,
    onRootRemove,
    onTreeUpdate,
    setTree,
  });

  useMoveTreeNodeEvent({
    treeId,
    onNodeAdd,
    onNodeRemove,
    onRootAdd,
    onRootRemove,
    onTreeUpdate,
    setTree,
  });

  return (
    <TreeContext.Provider
      value={{
        treeId,
        paddingLeft,
        paddingRight,
        rootOffset,
        nodeOffset,
        allFoldersAreExpanded: checkIfAllFoldersAreExpanded(tree.childNodes),
        allFoldersAreCollapsed: checkIfAllFoldersAreCollapsed(tree.childNodes),
        searchInput,
        sortBy,

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
      <div>
        <TreeRootNode onNodeUpdate={handleNodeUpdate} node={tree} />
      </div>
    </TreeContext.Provider>
  );
};

export default CollectionTree;
