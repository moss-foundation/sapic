import { DirDepthIndicator } from "../DirDepthIndicator";
import { TreeCollectionNode } from "../types";
import { sortByOrder } from "../utils";
import { TreeNode } from "./TreeNode";

interface TreeNodeChildrenProps {
  node: TreeCollectionNode;
  depth: number;
}

const TreeNodeChildren = ({ node, depth }: TreeNodeChildrenProps) => {
  const sortedChildNodes = sortByOrder(node.childNodes);

  return (
    <ul className="relative h-full">
      {node.kind === "Dir" && node.expanded && <DirDepthIndicator depth={depth} />}

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
  );
};

export default TreeNodeChildren;
