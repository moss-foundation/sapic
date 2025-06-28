import { TreeCollectionNode } from "../types";
import { TreeNode } from "./TreeNode";

interface TreeNodeChildrenProps {
  node: TreeCollectionNode;
  onNodeUpdate: (node: TreeCollectionNode) => void;
  depth: number;
}

const TreeNodeChildren = ({ node, onNodeUpdate, depth }: TreeNodeChildrenProps) => {
  return (
    <div className="contents">
      <ul className="h-full">
        {node.childNodes.map((childNode, index) => (
          <TreeNode
            parentNode={node}
            onNodeUpdate={onNodeUpdate}
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
