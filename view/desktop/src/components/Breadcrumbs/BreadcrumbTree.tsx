import { useState } from "react";

import { ProjectTreeNode } from "../ProjectTree/types";
import BreadcrumbNode from "./BreadcrumbNode";
import { closeAllNodesInTree, updateTreeNode } from "./utils";

interface BreadcrumbTreeProps {
  tree: ProjectTreeNode;
  projectId: string;
}

export const BreadcrumbTree = ({ tree: initialTree, projectId }: BreadcrumbTreeProps) => {
  const [tree, setTree] = useState<ProjectTreeNode>(closeAllNodesInTree(initialTree));

  const handleNodeUpdate = (updatedNode: ProjectTreeNode) => {
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
