import { useState } from "react";

import { ResourceNode } from "../ProjectTree/ResourcesTree/types";
import BreadcrumbNode from "./BreadcrumbNode";
import { closeAllNodesInTree } from "./transforms/closeAllNodesInTree";
import { updateTreeNode } from "./transforms/updateTreeNode";

interface BreadcrumbTreeProps {
  tree: ResourceNode;
  projectId: string;
}

export const BreadcrumbTree = ({ tree: initialTree, projectId }: BreadcrumbTreeProps) => {
  const [tree, setTree] = useState<ResourceNode>(closeAllNodesInTree(initialTree));

  const handleNodeUpdate = (updatedNode: ResourceNode) => {
    setTree(updateTreeNode(tree, updatedNode));
  };

  return (
    <ul>
      {tree.childNodes.map((childNode) => (
        <BreadcrumbNode key={childNode.id} projectId={projectId} node={childNode} onNodeUpdate={handleNodeUpdate} />
      ))}
    </ul>
  );
};

export default BreadcrumbTree;
