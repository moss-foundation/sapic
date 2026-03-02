import { Tree } from "@/lib/ui/Tree";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";

import { ProjectTreeNode } from "../types";
import { TreeNode } from "./TreeNode";

interface TreeNodeChildrenProps {
  node: ProjectTreeNode;
  depth?: number;
  offset?: number;
  treeOffset?: number;
}

const TreeNodeChildren = ({ node, depth = 1, offset, treeOffset }: TreeNodeChildrenProps) => {
  const sortedChildNodes = sortObjectsByOrder(node.childNodes);
  const dirDepthIndicatorOffset = depth && offset && treeOffset ? treeOffset + depth * offset : 0;

  return (
    <Tree.NodeChildren dirDepthIndicatorOffset={dirDepthIndicatorOffset}>
      {sortedChildNodes.map((childNode, index) => (
        <TreeNode
          parentNode={node}
          key={childNode.id}
          node={childNode}
          depth={depth + 1}
          isLastChild={index === node.childNodes.length - 1}
        />
      ))}
    </Tree.NodeChildren>
  );
};

export default TreeNodeChildren;
