import { Tree } from "@/lib/ui/Tree";

import { ResourceNode, ResourcesTree } from "../types";
import { ResourcesTreeNode } from "./ResourcesTreeNode";

interface ResourcesTreeChildrenProps {
  rootResourcesNodes: ResourceNode[];
  parentNode: ResourcesTree | ResourceNode;
  depth: number;
}

export const ResourcesTreeChildren = ({ rootResourcesNodes, parentNode, depth }: ResourcesTreeChildrenProps) => {
  return (
    <Tree.ListChildren>
      {rootResourcesNodes.map((node) => (
        <ResourcesTreeNode key={node.id} node={node} parentNode={parentNode} depth={depth + 1} />
      ))}
    </Tree.ListChildren>
  );
};
