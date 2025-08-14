import { useEffect, useState } from "react";

import { CollectionTreeContext } from "./CollectionTreeContext.tsx";
import { TreeRootNode } from "./TreeRootNode/TreeRootNode.tsx";
import { CollectionTreeProps, TreeCollectionRootNode } from "./types.ts";
import { checkIfAllFoldersAreCollapsed, checkIfAllFoldersAreExpanded } from "./utils/TreeRoot.ts";

export const CollectionTree = ({
  tree: initialTree,
  treePaddingLeft = 8,
  treePaddingRight = 8,
  nodeOffset = 12,
  searchInput,
  displayMode = "REQUEST_FIRST",
  showNodeOrders = false,
}: CollectionTreeProps) => {
  const [tree, setTree] = useState<TreeCollectionRootNode>(initialTree);

  useEffect(() => {
    setTree(initialTree);
  }, [initialTree]);

  return (
    <CollectionTreeContext.Provider
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
        displayMode,
        showNodeOrders,
      }}
    >
      <TreeRootNode node={tree} />
    </CollectionTreeContext.Provider>
  );
};

export default CollectionTree;
