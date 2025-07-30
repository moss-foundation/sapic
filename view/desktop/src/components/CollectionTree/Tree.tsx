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

  treePaddingLeft: 0,
  treePaddingRight: 0,
  nodeOffset: 0,
  allFoldersAreExpanded: false,
  allFoldersAreCollapsed: true,
  searchInput: undefined,
  sortBy: "none",
  displayMode: "REQUEST_FIRST",
  showNodeOrders: false,
});

export const CollectionTree = ({
  tree: initialTree,

  treePaddingLeft = 8,
  treePaddingRight = 8,
  nodeOffset = 16,
  searchInput,
  sortBy = "none",
  displayMode = "REQUEST_FIRST",
  showNodeOrders = false,
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

        treePaddingLeft,
        treePaddingRight,
        nodeOffset,
        allFoldersAreExpanded: checkIfAllFoldersAreExpanded(tree),
        allFoldersAreCollapsed: checkIfAllFoldersAreCollapsed(tree),
        searchInput,
        sortBy,
        displayMode,
        showNodeOrders,
      }}
    >
      <TreeRootNode node={tree} />
    </TreeContext.Provider>
  );
};

export default CollectionTree;
