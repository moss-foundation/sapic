import { TreeCollectionNode } from "../types";
import { sortByOrder } from "../utils2";
import { TreeNode } from "./TreeNode";

interface TreeNodeChildrenProps {
  node: TreeCollectionNode;
  depth: number;
}

const TreeNodeChildren = ({ node, depth }: TreeNodeChildrenProps) => {
  const sortedChildNodes = sortByOrder(node.childNodes);

  return (
    <div className="contents">
      <ul className="h-full">
        {sortedChildNodes.map((childNode, index) => (
          <TreeNode
            parentNode={node}
            key={childNode.id}
            node={childNode}
            depth={depth + 1}
            isLastChild={index === node.childNodes.length - 1}
          />
        ))}
      </ul>
    </div>
  );
};

export default TreeNodeChildren;
