import { createContext, useEffect, useState } from "react";

import { TreeRootNode } from "./TreeRootNode/TreeRootNode.tsx";
import { TreeCollectionRootNode, TreeContextProps, TreeProps } from "./types.ts";
import { checkIfAllFoldersAreCollapsed, checkIfAllFoldersAreExpanded } from "./utils/TreeRoot.ts";

export const TreeContext = createContext<TreeContextProps>({
  id: "",
  name: "",
  repository: undefined,
  order: undefined,
  picturePath: undefined,
  expanded: false,

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
}: TreeProps) => {
  const [tree, setTree] = useState<TreeCollectionRootNode>(initialTree);

  useEffect(() => {
    setTree(initialTree);
  }, [initialTree]);

  return (
    <TreeContext.Provider
      value={{
        id: initialTree.id,
        expanded: initialTree.expanded,
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
      <TreeRootNode node={tree} />
    </TreeContext.Provider>
  );
};

export default CollectionTree;
