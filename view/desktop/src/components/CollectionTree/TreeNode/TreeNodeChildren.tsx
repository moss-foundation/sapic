import { useContext } from "react";

import { TreeContext } from "../Tree";
import { hasDescendantWithSearchInput } from "../utils";
import { TreeNode } from "./TreeNode";

const TreeNodeChildren = ({ node, onNodeUpdate, depth }) => {
  const { searchInput } = useContext(TreeContext);
  const filteredChildNodes = searchInput
    ? node.childNodes.filter((childNode) => hasDescendantWithSearchInput(childNode, searchInput))
    : node.childNodes;

  return (
    <div className="contents">
      <ul className="h-full">
        {filteredChildNodes.map((childNode, index) => (
          <TreeNode
            parentNode={node}
            onNodeUpdate={onNodeUpdate}
            key={childNode.uniqueId}
            node={childNode}
            depth={depth + 1}
            isLastChild={index === filteredChildNodes.length - 1}
          />
        ))}
      </ul>
    </div>
  );
};

export default TreeNodeChildren;
