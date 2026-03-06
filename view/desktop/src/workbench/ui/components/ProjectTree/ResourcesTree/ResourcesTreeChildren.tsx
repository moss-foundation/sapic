import { Tree } from "@/lib/ui/Tree";

import { IResourcesTree, ResourceNode } from "../types";
import { ResourcesTreeNode } from "./ResourcesTreeNode";

interface ResourcesTreeChildrenProps {
  rootResourcesNodes: ResourceNode[];
  parentNode: IResourcesTree | ResourceNode;
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
