import { createContext, useEffect, useState } from "react";

import { TreeRootNode } from "./TreeRootNode/TreeRootNode.tsx";
import { TreeCollectionNode, TreeCollectionRootNode, TreeContextProps, TreeProps } from "./types.ts";
import {
  checkIfAllFoldersAreCollapsed,
  checkIfAllFoldersAreExpanded,
  updateNodeInTree,
} from "./utils/TreeRootUtils.ts";

export const TreeContext = createContext<TreeContextProps>({
  id: "",
  name: "",
  repository: undefined,
  order: undefined,
  picturePath: undefined,

  paddingLeft: 0,
  paddingRight: 0,
  rootOffset: 0,
  nodeOffset: 0,
  allFoldersAreExpanded: false,
  allFoldersAreCollapsed: true,
  searchInput: undefined,
  sortBy: "none",
  displayMode: "REQUEST_FIRST",
});

export const CollectionTree = ({
  tree: initialTree,

  paddingLeft = 8,
  paddingRight = 8,
  rootOffset = 8,
  nodeOffset = 16,
  searchInput,
  sortBy = "none",
  displayMode = "REQUEST_FIRST",

  onTreeUpdate,
}: TreeProps) => {
  const [tree, setTree] = useState<TreeCollectionRootNode>(initialTree);

  const handleNodeUpdate = (updatedNode: TreeCollectionNode) => {
    setTree((prev) => {
      const newTree = updateNodeInTree(prev, updatedNode);
      onTreeUpdate?.(newTree);
      return newTree;
    });
  };

  const handleRootNodeUpdate = (updatedNode: TreeCollectionRootNode) => {
    setTree(updatedNode);
    onTreeUpdate?.(updatedNode);
  };

  useEffect(() => {
    setTree(initialTree);
  }, [initialTree]);

  // useMoveTreeNodeEvent({
  //   treeId: initialTree.id,
  //   onNodeAdd,
  //   onNodeRemove,
  //   onRootAdd,
  //   onRootRemove,
  //   onTreeUpdate,
  //   setTree,
  // });

  return (
    <TreeContext.Provider
      value={{
        id: initialTree.id,
        name: initialTree.name,
        repository: initialTree.repository,
        order: initialTree.order,
        picturePath: initialTree.picturePath,

        paddingLeft,
        paddingRight,
        rootOffset,
        nodeOffset,
        allFoldersAreExpanded: checkIfAllFoldersAreExpanded(tree),
        allFoldersAreCollapsed: checkIfAllFoldersAreCollapsed(tree),
        searchInput,
        sortBy,
        displayMode,
      }}
    >
      <TreeRootNode onNodeUpdate={handleNodeUpdate} onRootNodeUpdate={handleRootNodeUpdate} node={tree} />
    </TreeContext.Provider>
  );
};

export default CollectionTree;
