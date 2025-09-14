import { Tree } from "@/lib/ui/Tree";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";

import { ProjectTreeNode } from "../types";
import { TreeNode } from "./TreeNode";

interface TreeNodeChildrenProps {
  node: ProjectTreeNode;
  depth: number;
}

const TreeNodeChildren = ({ node, depth }: TreeNodeChildrenProps) => {
  const sortedChildNodes = sortObjectsByOrder(node.childNodes);

  return (
    <Tree.NodeChildren depth={depth}>
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
