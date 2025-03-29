import { useState } from "react";

import { NodeProps, TreeNodeProps } from "../Tree/types";
import { prepareCollectionForTree, updateTreeNode } from "../Tree/utils";
import BreadcrumbNode from "./BreadcrumbNode";

interface BreadcrumbTreeProps {
  tree: NodeProps;
  onNodeClick: (node: NodeProps) => void;
}

export const BreadcrumbTree = ({ tree: initialTree, onNodeClick: onNodeClickCallback }: BreadcrumbTreeProps) => {
  const [tree, setTree] = useState<TreeNodeProps>(prepareCollectionForTree(initialTree, false));

  const handleNodeUpdate = (updatedNode: TreeNodeProps) => {
    setTree(updateTreeNode(tree, updatedNode));

    onNodeClickCallback?.(updatedNode);
  };

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
