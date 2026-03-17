import { Tree } from "@/lib/ui/Tree";

import { NODE_OFFSET } from "../constants";
import { ResourcesTreeRoot } from "../TreeRoot/types";
import { ResourcesTreeNode } from "./ResourcesTreeNode";
import { ResourceNode } from "./types";

interface ResourcesTreeChildrenProps {
  rootResourcesNodes: ResourceNode[];
  parentNode: ResourcesTreeRoot | ResourceNode;
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
