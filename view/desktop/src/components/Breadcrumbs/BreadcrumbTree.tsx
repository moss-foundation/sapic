/* eslint-disable */
import { useState } from "react";

import { TreeCollectionNode } from "../CollectionTree/types";
import BreadcrumbNode from "./BreadcrumbNode";

interface BreadcrumbTreeProps {
  tree: TreeCollectionNode;
  onNodeClick: (node: TreeCollectionNode) => void;
}

export const BreadcrumbTree = ({ tree: initialTree, onNodeClick: onNodeClickCallback }: BreadcrumbTreeProps) => {
  const [tree, setTree] = useState<TreeCollectionNode>(initialTree);

  const handleNodeUpdate = (updatedNode: TreeCollectionNode) => {
    // setTree(updateNodeInTree(tree, updatedNode));

    onNodeClickCallback?.(updatedNode);
  };

  if (!tree.childNodes || tree.childNodes.length === 0) return null;

  return (
    <ul>
      {tree.childNodes.map((childNode) => (
        <BreadcrumbNode
          key={childNode.id}
          node={childNode}
          onNodeUpdate={handleNodeUpdate}
          onNodeClickCallback={onNodeClickCallback}
        />
      ))}
    </ul>
  );
};
export default BreadcrumbTree;
