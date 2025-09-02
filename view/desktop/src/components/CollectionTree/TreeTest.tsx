import { WorkspaceMode } from "@repo/moss-workspace";

import { Tree } from "../Tree";
import { CollectionTreeContextTest } from "./CollectionTreeContextTest";
import { TreeRootNodeTest } from "./TreeRootNode/TreeRootNodeTest";
import { TreeCollectionRootNode } from "./types";

export const CollectionTreeTest = ({
  tree: initialTree,
  treePaddingLeft = 0,
  treePaddingRight = 0,
  ...props
}: {
  tree: TreeCollectionRootNode;
  treePaddingLeft?: number;
  treePaddingRight?: number;
  nodeOffset?: number;
  searchInput?: string;
  displayMode?: WorkspaceMode;
  showNodeOrders?: boolean;
}) => {
  return (
    <CollectionTreeContextTest.Provider
      value={{
        id: initialTree.id,
        treePaddingLeft,
        treePaddingRight,
        allFoldersAreExpanded: true,
        allFoldersAreCollapsed: false,
        ...props,
        something: "test",
      }}
    >
      <Tree>
        <TreeRootNodeTest node={initialTree} />
      </Tree>
    </CollectionTreeContextTest.Provider>
  );
};

export default CollectionTreeTest;
