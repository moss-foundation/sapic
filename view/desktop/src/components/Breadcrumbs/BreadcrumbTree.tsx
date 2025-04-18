import { useState } from "react";

import { NodeProps, TreeNodeProps } from "../Tree/types";
import { collapseAllNodes, prepareCollectionForTree, updateTreeNode } from "../Tree/utils";
import BreadcrumbNode from "./BreadcrumbNode";

interface BreadcrumbTreeProps {
  tree: NodeProps;
  onNodeClick: (node: NodeProps) => void;
}

export const BreadcrumbTree = ({ tree: initialTree, onNodeClick: onNodeClickCallback }: BreadcrumbTreeProps) => {
  const [tree, setTree] = useState<TreeNodeProps>(collapseAllNodes(prepareCollectionForTree(initialTree, false)));

  const handleNodeUpdate = (updatedNode: TreeNodeProps) => {
    setTree(updateTreeNode(tree, updatedNode));

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
