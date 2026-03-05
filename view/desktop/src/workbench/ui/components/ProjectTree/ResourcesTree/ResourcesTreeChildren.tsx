import { Tree } from "@/lib/ui/Tree";

import { ResourceNode } from "../types";
import { ResourcesTreeNode } from "./ResourcesTreeNode";

interface ResourcesTreeChildrenProps {
  rootResourcesNodes: ResourceNode[];
  depth: number;
}

export const ResourcesTreeChildren = ({ rootResourcesNodes, depth }: ResourcesTreeChildrenProps) => {
  return (
    <Tree.ListChildren>
      {rootResourcesNodes.map((node) => (
        <ResourcesTreeNode key={node.id} node={node} depth={depth + 1} />
      ))}
    </Tree.ListChildren>
  );
};
