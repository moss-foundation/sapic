import { useState } from "react";

import { TreeCollectionNode } from "../CollectionTree/types";
import BreadcrumbNode from "./BreadcrumbNode";
import { closeAllNodesInTree, updateTreeNode } from "./utils";

interface BreadcrumbTreeProps {
  tree: TreeCollectionNode;
  collectionId: string;
}

export const BreadcrumbTree = ({ tree: initialTree, collectionId }: BreadcrumbTreeProps) => {
  const [tree, setTree] = useState<TreeCollectionNode>(closeAllNodesInTree(initialTree));

  const handleNodeUpdate = (updatedNode: TreeCollectionNode) => {
    setTree(updateTreeNode(tree, updatedNode));
  };

  return (
    <ul>
      {tree.childNodes.map((childNode) => (
        <BreadcrumbNode
          key={childNode.id}
          collectionId={collectionId}
          node={childNode}
          onNodeUpdate={handleNodeUpdate}
        />
      ))}
    </ul>
  );
};
export default BreadcrumbTree;
