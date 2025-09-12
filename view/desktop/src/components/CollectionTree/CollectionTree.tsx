import { CollectionTreeContext } from "./CollectionTreeContext.tsx";
import { TreeRootNode } from "./TreeRootNode/TreeRootNode.tsx";
import { CollectionTreeProps } from "./types.ts";
import { checkIfAllFoldersAreCollapsed, checkIfAllFoldersAreExpanded } from "./utils/TreeRoot.ts";

export const CollectionTree = ({
  tree,
  treePaddingLeft = 8,
  treePaddingRight = 8,
  nodeOffset = 12,
  searchInput,
  displayMode = "REQUEST_FIRST",
  showOrders = true,
}: CollectionTreeProps) => {
  return (
    <CollectionTreeContext.Provider
      value={{
        id: tree.id,
        name: tree.name,
        order: tree.order ?? 0,
        iconPath: tree.iconPath,
        treePaddingLeft,
        treePaddingRight,
        nodeOffset,
        allFoldersAreExpanded: checkIfAllFoldersAreExpanded(tree),
        allFoldersAreCollapsed: checkIfAllFoldersAreCollapsed(tree),
        searchInput: searchInput ?? "",
        displayMode,
        showOrders,
      }}
    >
      <TreeRootNode node={tree} />
    </CollectionTreeContext.Provider>
  );
};

export default CollectionTree;
