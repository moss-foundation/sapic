import { Tree } from "@/lib/ui/Tree";

import { NODE_OFFSET } from "../constants";
import { IResourcesTree, ResourceNode } from "../types";
import { ResourcesTreeNode } from "./ResourcesTreeNode";

interface ResourcesTreeChildrenProps {
  rootResourcesNodes: ResourceNode[];
  parentNode: IResourcesTree | ResourceNode;
  depth: number;
}

export const ResourcesTreeChildren = ({ rootResourcesNodes, parentNode, depth }: ResourcesTreeChildrenProps) => {
  return (
    <Tree.NodeChildren depth={depth} nodeOffset={NODE_OFFSET}>
      {rootResourcesNodes.map((node) => (
        <ResourcesTreeNode key={node.id} node={node} parentNode={parentNode} depth={depth + 1} />
      ))}
    </Tree.NodeChildren>
  );
};
